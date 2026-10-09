use crate::github::Content;
use eyre::{eyre, Context, Result};
use http::header::{AUTHORIZATION, USER_AGENT};
use http::{Method, StatusCode};
use serde::de::DeserializeOwned;

pub use crate::{assets::*, chain::*, paths::*};

const VERSION: &str = env!("CARGO_PKG_VERSION");
// Always use the master branch to get the latest registry data
const GIT_REF: &str = "master";
const RAW_FILE_REPO_URL: &str = "https://raw.githubusercontent.com/cosmos/chain-registry";
const REPO_URL: &str = "https://api.github.com/repos/cosmos/chain-registry/contents";

/// Environment variable consulted for a GitHub token by [`RegistryClient::new`].
const TOKEN_ENV_VAR: &str = "CHAIN_REGISTRY_GITHUB_TOKEN";

/// Prefix identifying GitHub API requests, which are the only ones the token is sent to.
const API_HOST_PREFIX: &str = "https://api.github.com/";

/// A client for the Cosmos chain registry with optional GitHub authentication.
///
/// GitHub's unauthenticated API allows only 60 requests per hour, which the listing endpoints
/// (`list_chains`, `list_paths`, `list_testnets`) and the [`RegistryCache`] constructor can exhaust.
/// Pass a token with [`RegistryClient::with_token`], or set `CHAIN_REGISTRY_GITHUB_TOKEN` and use
/// [`RegistryClient::new`] to raise the limit to 5,000 requests per hour. The token only needs read
/// access to repository contents, and is only sent to the `api.github.com` requests that need it.
///
/// # Examples
///
/// ```no_run
/// use cosmos_chain_registry::get::RegistryClient;
///
/// # async fn run() -> eyre::Result<()> {
/// // Explicit token for this client.
/// let client = RegistryClient::with_token(std::env::var("MY_GITHUB_TOKEN")?);
/// let chains = client.list_chains().await?;
///
/// // Or read CHAIN_REGISTRY_GITHUB_TOKEN from the environment.
/// let client = RegistryClient::new();
/// let chain = client.get_chain("osmosis").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct RegistryClient {
    http: reqwest::Client,
    token: Option<String>,
}

/// Redacts the token so a client is safe to log.
impl std::fmt::Debug for RegistryClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistryClient")
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .finish_non_exhaustive()
    }
}

impl Default for RegistryClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryClient {
    /// Creates a client that reads a token from `CHAIN_REGISTRY_GITHUB_TOKEN`. Falls back to
    /// anonymous access when it is unset or blank.
    pub fn new() -> Self {
        Self::with_optional_token(token_from(|key| std::env::var(key).ok()))
    }

    /// Creates an anonymous client that sends no token.
    pub fn anonymous() -> Self {
        Self::with_optional_token(None)
    }

    /// Creates a client that authenticates GitHub API requests with `token`.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self::with_optional_token(Some(token.into()))
    }

    fn with_optional_token(token: Option<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token
                .map(|token| token.trim().to_string())
                .filter(|token| !token.is_empty()),
        }
    }

    fn build_request(&self, url: &str) -> Result<reqwest::Request> {
        let mut builder = self
            .http
            .request(Method::GET, url)
            .header(USER_AGENT, format!("ocular/{VERSION}"));
        // Only the rate-limited API requests need the token; raw content must not receive it.
        if let Some(token) = self
            .token
            .as_deref()
            .filter(|_| url.starts_with(API_HOST_PREFIX))
        {
            builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
        }
        Ok(builder.build()?)
    }

    async fn get(&self, url: &str) -> Result<String> {
        let response = self.http.execute(self.build_request(url)?).await?;
        let status = response.status();
        if !status.is_success() {
            return Err(github_status_error(status, url));
        }
        Ok(response.text().await?)
    }

    async fn get_file_content(&self, r#ref: &str, path: &str) -> Result<Option<String>> {
        let url = format!("{RAW_FILE_REPO_URL}/{ref}/{path}");
        let response = self.http.execute(self.build_request(&url)?).await?;
        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            return Err(github_status_error(status, &url));
        }

        Ok(Some(
            response
                .text()
                .await
                .wrap_err("error getting remote file content")?,
        ))
    }

    /// Gets a list of chain names from the registry.
    pub async fn list_chains(&self) -> Result<Vec<String>> {
        let url = format!("{REPO_URL}?ref={GIT_REF}");
        let json = self.get(&url).await?;
        let contents: Vec<Content> = serde_json::from_str(json.as_str())?;

        Ok(contents
            .iter()
            .filter(|c| c.type_field == "dir" && !c.name.starts_with('_') && c.name != ".github")
            .map(|c| c.clone().name)
            .collect())
    }

    /// Gets a list of testnet chain names from the registry.
    pub async fn list_testnets(&self) -> Result<Vec<String>> {
        let url = format!("{REPO_URL}/testnets?ref={GIT_REF}");
        let json = self.get(&url).await?;
        let contents: Vec<Content> = serde_json::from_str(json.as_str())?;

        Ok(contents
            .iter()
            .filter(|c| {
                c.type_field == "dir" && !c.name.starts_with('_') && !c.name.starts_with('.')
            })
            .map(|c| c.clone().name)
            .collect())
    }

    /// Gets a list of path names from the registry in the form <chain_a>-<chain_b>.
    pub async fn list_paths(&self) -> Result<Vec<String>> {
        let url = format!("{REPO_URL}/_IBC?ref={GIT_REF}");
        let json = self.get(&url).await?;
        let contents: Vec<Content> = serde_json::from_str(json.as_str())?;

        Ok(contents
            .iter()
            .filter(|c| {
                c.type_field == "file" && !c.name.starts_with('_') && c.name.ends_with(".json")
            })
            .map(|c| c.name[..c.name.len() - ".json".len()].to_string())
            .collect())
    }

    /// Retrieves the deserialized `assets.json` for a given chain. The result will contain
    /// `None` if the there is no `assets.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_assets(&self, name: &str) -> Result<Option<AssetList>> {
        let path = format!("{name}/assetlist.json");
        let data = self.get_file_content(GIT_REF, &path).await?;

        match data {
            Some(d) => Ok(parse_json(d).await),
            None => Ok(None),
        }
    }

    /// Retrieves the deserialized `chain.json` for a given chain. The result will contain
    /// `None` if the there is no `chain.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_chain(&self, name: &str) -> Result<Option<ChainInfo>> {
        let path = format!("{name}/chain.json");
        let data = self.get_file_content(GIT_REF, &path).await?;

        match data {
            Some(d) => Ok(parse_json(d).await),
            None => Ok(None),
        }
    }

    /// Retrieves the deserialized IBC path json for a given pair of chains. The result will contain
    /// `None` if the there is no path present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_path(&self, chain_a: &str, chain_b: &str) -> Result<Option<IBCPath>> {
        // path names order the chain names alphabetically
        let path = format!(
            "_IBC/{}-{}.json",
            chain_a.min(chain_b),
            chain_a.max(chain_b)
        );
        let data = self.get_file_content(GIT_REF, &path).await?;

        match data {
            Some(d) => Ok(parse_json(d).await),
            None => Ok(None),
        }
    }

    /// Retrieves the gRPC endpoint addresses for a given chain. The result will contain
    /// `None` if the there is no `chain.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_grpc_endpoints(&self, name: &str) -> Result<Option<Vec<String>>> {
        Ok(self.get_chain(name).await?.map(|chain| {
            chain
                .apis
                .grpc
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
    }

    /// Retrieves the REST/LCD endpoint addresses for a given chain. The result will contain
    /// `None` if the there is no `chain.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_rest_endpoints(&self, name: &str) -> Result<Option<Vec<String>>> {
        Ok(self.get_chain(name).await?.map(|chain| {
            chain
                .apis
                .rest
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
    }

    /// Retrieves the RPC endpoint addresses for a given chain. The result will contain
    /// `None` if the there is no `chain.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_rpc_endpoints(&self, name: &str) -> Result<Option<Vec<String>>> {
        Ok(self.get_chain(name).await?.map(|chain| {
            chain
                .apis
                .rpc
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
    }

    /// Retrieves the EVM JSON-RPC endpoint addresses for a given chain. The result will contain
    /// `None` if the there is no `chain.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    ///   [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_evm_rpc_endpoints(&self, name: &str) -> Result<Option<Vec<String>>> {
        Ok(self.get_chain(name).await?.map(|chain| {
            chain
                .apis
                .evm_http_jsonrpc
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
    }
}

/// Reads a GitHub token from the environment via `lookup`, trimming blanks.
fn token_from<F: Fn(&str) -> Option<String>>(lookup: F) -> Option<String> {
    lookup(TOKEN_ENV_VAR)
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
}

/// Builds an error for a non-success GitHub response without echoing request headers.
fn github_status_error(status: StatusCode, url: &str) -> eyre::Report {
    let hint = match status.as_u16() {
        401 | 403 => {
            " (check the GitHub token passed to RegistryClient::with_token or CHAIN_REGISTRY_GITHUB_TOKEN)"
        }
        _ => "",
    };
    eyre!("GitHub request to {url} failed with HTTP {status}{hint}")
}

/// Gets a list of chain names from the registry.
pub async fn list_chains() -> Result<Vec<String>> {
    RegistryClient::new().list_chains().await
}

/// Gets a list of testnet chain names from the registry.
pub async fn list_testnets() -> Result<Vec<String>> {
    RegistryClient::new().list_testnets().await
}

/// Gets a list of path names from the registry in the form <chain_a>-<chain_b>.
pub async fn list_paths() -> Result<Vec<String>> {
    RegistryClient::new().list_paths().await
}

/// Retrieves the deserialized `assets.json` for a given chain.
pub async fn get_assets(name: &str) -> Result<Option<AssetList>> {
    RegistryClient::new().get_assets(name).await
}

/// Retrieves the deserialized `chain.json` for a given chain.
pub async fn get_chain(name: &str) -> Result<Option<ChainInfo>> {
    RegistryClient::new().get_chain(name).await
}

/// Retrieves the deserialized IBC path json for a given pair of chains.
pub async fn get_path(chain_a: &str, chain_b: &str) -> Result<Option<IBCPath>> {
    RegistryClient::new().get_path(chain_a, chain_b).await
}

/// Retrieves the gRPC endpoint addresses for a given chain.
pub async fn get_grpc_endpoints(name: &str) -> Result<Option<Vec<String>>> {
    RegistryClient::new().get_grpc_endpoints(name).await
}

/// Retrieves the REST/LCD endpoint addresses for a given chain.
pub async fn get_rest_endpoints(name: &str) -> Result<Option<Vec<String>>> {
    RegistryClient::new().get_rest_endpoints(name).await
}

/// Retrieves the RPC endpoint addresses for a given chain.
pub async fn get_rpc_endpoints(name: &str) -> Result<Option<Vec<String>>> {
    RegistryClient::new().get_rpc_endpoints(name).await
}

/// Retrieves the EVM JSON-RPC endpoint addresses for a given chain.
pub async fn get_evm_rpc_endpoints(name: &str) -> Result<Option<Vec<String>>> {
    RegistryClient::new().get_evm_rpc_endpoints(name).await
}

async fn parse_json<T>(data: String) -> Option<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(&data).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[test]
    fn client_attaches_bearer_token_when_configured() {
        let request = RegistryClient::with_token("ghp_example")
            .build_request("https://api.github.com/repos/cosmos/chain-registry/contents")
            .expect("build request");
        assert_eq!(
            request
                .headers()
                .get(AUTHORIZATION)
                .expect("authorization header"),
            "Bearer ghp_example"
        );

        let anonymous = RegistryClient::anonymous()
            .build_request("https://api.github.com/repos/cosmos/chain-registry/contents")
            .expect("build request");
        assert!(anonymous.headers().get(AUTHORIZATION).is_none());
    }

    #[test]
    fn blank_token_is_ignored() {
        let request = RegistryClient::with_token("   ")
            .build_request("https://api.github.com/")
            .expect("build request");
        assert!(request.headers().get(AUTHORIZATION).is_none());
    }

    #[test]
    fn token_is_only_sent_to_the_api_host() {
        let client = RegistryClient::with_token("ghp_example");
        let raw = client
            .build_request(
                "https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/chain.json",
            )
            .expect("build request");
        assert!(raw.headers().get(AUTHORIZATION).is_none());
    }

    #[test]
    fn debug_redacts_token() {
        let rendered = format!("{:?}", RegistryClient::with_token("ghp_secret"));
        assert!(rendered.contains("redacted"));
        assert!(!rendered.contains("ghp_secret"));
    }

    #[test]
    fn token_from_trims_and_ignores_blank() {
        assert_eq!(
            token_from(|_| Some("  abc  ".to_string())),
            Some("abc".to_string())
        );
        assert_eq!(token_from(|_| Some("   ".to_string())), None);
        assert_eq!(token_from(|_| None), None);
        assert_eq!(
            token_from(|key| (key == TOKEN_ENV_VAR).then(|| "tok".to_string())),
            Some("tok".to_string())
        );
    }

    #[assay]
    async fn gets_content_from_registry() {
        let result = RegistryClient::new()
            .get_file_content(GIT_REF, "cosmoshub/chain.json")
            .await;

        result.unwrap().unwrap();
    }

    #[assay]
    async fn parses_chain_info() {
        let result = RegistryClient::new()
            .get_file_content(GIT_REF, "cosmoshub/chain.json")
            .await
            .unwrap()
            .unwrap();
        let result = parse_json::<ChainInfo>(result).await;

        result.unwrap();
    }

    #[assay]
    async fn gets_chain() {
        let result = get_chain("cosmoshub").await;

        result.unwrap().unwrap();
    }

    #[assay]
    async fn lists_chains() {
        list_chains().await.unwrap();
    }

    #[assay]
    async fn lists_paths() {
        let paths = list_paths().await.unwrap();
        assert!(!paths.is_empty());
        paths
            .iter()
            .for_each(|path| assert!(!path.ends_with(".json")))
    }

    #[assay]
    async fn gets_path_in_order() {
        let chain_a = "cosmoshub";
        let chain_b = "osmosis";
        let result = get_path(chain_a, chain_b).await.unwrap().unwrap();
        assert_eq!(result.chain_1.chain_name, "cosmoshub");
        assert_eq!(result.chain_2.chain_name, "osmosis");
    }

    #[assay]
    async fn gets_path_out_of_order() {
        let chain_a = "cosmoshub";
        let chain_b = "osmosis";
        let result = get_path(chain_b, chain_a).await.unwrap().unwrap();
        assert_eq!(result.chain_1.chain_name, "cosmoshub");
        assert_eq!(result.chain_2.chain_name, "osmosis");
    }

    #[assay]
    async fn get_path_not_present_returns_none() {
        let chain_a = "fake";
        let chain_b = "osmosis";
        let result = get_path(chain_b, chain_a).await.unwrap();
        assert!(result.is_none())
    }
}
