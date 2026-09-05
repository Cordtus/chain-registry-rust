/// Support for extending registry data with additional information (such as
/// private endpoints) that cannot be made public in the chain registry.
///
/// Data is read from a local JSON file keyed by chain name. Overrides only ever
/// extend an existing chain's information; they never create new chains.
use crate::{
    chain::{ChainInfo, EvmRpc, Grpc, GrpcWeb, Rest, Rpc, Wss},
    get::get_chain,
};
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// A set of additional endpoints for a single chain
#[derive(Default, Debug, Clone, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct ChainOverrides {
    pub grpc: Vec<EndpointValue>,
    pub rest: Vec<EndpointValue>,
    pub rpc: Vec<EndpointValue>,
    pub wss: Vec<EndpointValue>,
    pub grpc_web: Vec<EndpointValue>,
    pub evm_http_jsonrpc: Vec<EndpointValue>,
}

/// An endpoint value, written either as a bare address string or a full object
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EndpointValue {
    Address(String),
    Full(Endpoint),
}

/// A fully-specified endpoint
#[derive(Default, Debug, Clone, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Endpoint {
    pub address: String,
    pub provider: Option<String>,
    pub archive: Option<bool>,
}

/// Additional chain information loaded from a local file, keyed by chain name.
///
/// # Example file format
///
/// ```json
/// {
///   "secretnetwork": {
///     "grpc": ["grpc.secretnetwork.pathrocknetwork.org:443"],
///     "rest": ["https://api.secretnetwork.pathrocknetwork.org"],
///     "rpc": ["https://rpc.secretnetwork.pathrocknetwork.org"],
///     "wss": ["wss://ws.secretnetwork.example"],
///     "grpc_web": ["https://grpc-web.secretnetwork.example"],
///     "evm_http_jsonrpc": ["https://evm.secretnetwork.example"]
///   }
/// }
/// ```
#[derive(Default, Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct LocalOverrides {
    #[serde(flatten)]
    pub chains: HashMap<String, ChainOverrides>,
}

impl LocalOverrides {
    /// Reads a local overrides file. The file must be valid JSON in the format
    /// described on [`LocalOverrides`].
    pub fn from_file(path: impl AsRef<Path>) -> Result<LocalOverrides> {
        let data = std::fs::read_to_string(path).wrap_err("error reading local overrides file")?;
        Self::parse(&data)
    }

    /// Parses local overrides from a JSON string.
    pub fn parse(data: &str) -> Result<LocalOverrides> {
        serde_json::from_str(data).wrap_err("error parsing local overrides")
    }
}

impl From<&EndpointValue> for Grpc {
    fn from(value: &EndpointValue) -> Self {
        let (address, provider, archive) = endpoint_parts(value);
        Grpc {
            address,
            provider,
            archive,
        }
    }
}

impl From<&EndpointValue> for Rest {
    fn from(value: &EndpointValue) -> Self {
        let (address, provider, archive) = endpoint_parts(value);
        Rest {
            address,
            provider,
            archive,
        }
    }
}

impl From<&EndpointValue> for Rpc {
    fn from(value: &EndpointValue) -> Self {
        let (address, provider, archive) = endpoint_parts(value);
        Rpc {
            address,
            provider,
            archive,
        }
    }
}

impl From<&EndpointValue> for Wss {
    fn from(value: &EndpointValue) -> Self {
        let (address, provider, archive) = endpoint_parts(value);
        Wss {
            address,
            provider,
            archive,
        }
    }
}

impl From<&EndpointValue> for GrpcWeb {
    fn from(value: &EndpointValue) -> Self {
        let (address, provider, archive) = endpoint_parts(value);
        GrpcWeb {
            address,
            provider,
            archive,
        }
    }
}

impl From<&EndpointValue> for EvmRpc {
    fn from(value: &EndpointValue) -> Self {
        let (address, provider, archive) = endpoint_parts(value);
        EvmRpc {
            address,
            provider,
            archive,
        }
    }
}

fn endpoint_parts(value: &EndpointValue) -> (String, Option<String>, Option<bool>) {
    match value {
        EndpointValue::Address(address) => (address.clone(), None, None),
        EndpointValue::Full(endpoint) => (
            endpoint.address.clone(),
            endpoint.provider.clone(),
            endpoint.archive,
        ),
    }
}

/// Retrieves the deserialized `chain.json` for a chain and merges any additional
/// endpoints found in the local overrides file into its `apis`. Existing
/// registry endpoints are preserved; additional endpoints with the same address
/// are de-duplicated.
///
/// The result will contain `None` if there is no `chain.json` present for the
/// chain. Overrides never create a new chain for a chain absent from the
/// registry.
pub async fn get_chain_with_overrides(
    name: &str,
    overrides: &LocalOverrides,
) -> Result<Option<ChainInfo>> {
    let Some(mut chain) = get_chain(name).await? else {
        return Ok(None);
    };
    let Some(chain_overrides) = overrides.chains.get(name) else {
        return Ok(Some(chain));
    };

    for value in &chain_overrides.grpc {
        let endpoint = Grpc::from(value);
        if !chain
            .apis
            .grpc
            .iter()
            .any(|existing| existing.address == endpoint.address)
        {
            chain.apis.grpc.push(endpoint);
        }
    }
    for value in &chain_overrides.rest {
        let endpoint = Rest::from(value);
        if !chain
            .apis
            .rest
            .iter()
            .any(|existing| existing.address == endpoint.address)
        {
            chain.apis.rest.push(endpoint);
        }
    }
    for value in &chain_overrides.rpc {
        let endpoint = Rpc::from(value);
        if !chain
            .apis
            .rpc
            .iter()
            .any(|existing| existing.address == endpoint.address)
        {
            chain.apis.rpc.push(endpoint);
        }
    }
    for value in &chain_overrides.wss {
        let endpoint = Wss::from(value);
        if !chain
            .apis
            .wss
            .iter()
            .any(|existing| existing.address == endpoint.address)
        {
            chain.apis.wss.push(endpoint);
        }
    }
    for value in &chain_overrides.grpc_web {
        let endpoint = GrpcWeb::from(value);
        if !chain
            .apis
            .grpc_web
            .iter()
            .any(|existing| existing.address == endpoint.address)
        {
            chain.apis.grpc_web.push(endpoint);
        }
    }
    for value in &chain_overrides.evm_http_jsonrpc {
        let endpoint = EvmRpc::from(value);
        if !chain
            .apis
            .evm_http_jsonrpc
            .iter()
            .any(|existing| existing.address == endpoint.address)
        {
            chain.apis.evm_http_jsonrpc.push(endpoint);
        }
    }

    Ok(Some(chain))
}

/// Retrieves the gRPC endpoint addresses for a chain, extended with any
/// additional endpoints from the local overrides file.
pub async fn get_grpc_endpoints_with_overrides(
    name: &str,
    overrides: &LocalOverrides,
) -> Result<Option<Vec<String>>> {
    Ok(get_chain_with_overrides(name, overrides)
        .await?
        .map(|chain| {
            chain
                .apis
                .grpc
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
}

/// Retrieves the REST/LCD endpoint addresses for a chain, extended with any
/// additional endpoints from the local overrides file.
pub async fn get_rest_endpoints_with_overrides(
    name: &str,
    overrides: &LocalOverrides,
) -> Result<Option<Vec<String>>> {
    Ok(get_chain_with_overrides(name, overrides)
        .await?
        .map(|chain| {
            chain
                .apis
                .rest
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
}

/// Retrieves the RPC endpoint addresses for a chain, extended with any
/// additional endpoints from the local overrides file.
pub async fn get_rpc_endpoints_with_overrides(
    name: &str,
    overrides: &LocalOverrides,
) -> Result<Option<Vec<String>>> {
    Ok(get_chain_with_overrides(name, overrides)
        .await?
        .map(|chain| {
            chain
                .apis
                .rpc
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
}

/// Retrieves the EVM JSON-RPC endpoint addresses for a chain, extended with any
/// additional endpoints from the local overrides file.
pub async fn get_evm_rpc_endpoints_with_overrides(
    name: &str,
    overrides: &LocalOverrides,
) -> Result<Option<Vec<String>>> {
    Ok(get_chain_with_overrides(name, overrides)
        .await?
        .map(|chain| {
            chain
                .apis
                .evm_http_jsonrpc
                .into_iter()
                .map(|endpoint| endpoint.address)
                .collect()
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_overrides() -> LocalOverrides {
        LocalOverrides::parse(
            r#"{
                "secretnetwork": {
                    "grpc": ["grpc.secretnetwork.pathrocknetwork.org:443"],
                    "rest": ["https://api.secretnetwork.pathrocknetwork.org"],
                    "rpc": ["https://rpc.secretnetwork.pathrocknetwork.org"]
                },
                "nonexistent_chain_xyz": {
                    "rpc": ["https://rpc.example.com"]
                }
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn parses_local_overrides() {
        let overrides = sample_overrides();
        assert_eq!(overrides.chains.len(), 2);
        let secret = &overrides.chains["secretnetwork"];
        assert_eq!(
            secret.grpc[0],
            EndpointValue::Address("grpc.secretnetwork.pathrocknetwork.org:443".to_string())
        );
    }

    #[test]
    fn parses_full_endpoint_objects() {
        let overrides = LocalOverrides::parse(
            r#"{
                "secretnetwork": {
                    "grpc": [{"address": "example.com:443", "provider": "example", "archive": true}]
                }
            }"#,
        )
        .unwrap();
        let secret = &overrides.chains["secretnetwork"];
        match &secret.grpc[0] {
            EndpointValue::Full(endpoint) => {
                assert_eq!(endpoint.address, "example.com:443");
                assert_eq!(endpoint.provider.as_deref(), Some("example"));
                assert_eq!(endpoint.archive, Some(true));
            }
            _ => panic!("expected full endpoint"),
        }
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(LocalOverrides::parse("not json").is_err());
    }
}
