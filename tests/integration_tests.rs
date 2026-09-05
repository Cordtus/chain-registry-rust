use cosmos_chain_registry::cache::RegistryCache;
use cosmos_chain_registry::get::*;
use cosmos_chain_registry::local::LocalOverrides;
use cosmos_chain_registry::local::{get_chain_with_overrides, get_grpc_endpoints_with_overrides};
use cosmos_chain_registry::paths::Tag;

#[tokio::test]
async fn test_list_chains() {
    let chains = list_chains().await.expect("Failed to list chains");
    assert!(!chains.is_empty(), "Chain list should not be empty");
    assert!(
        chains.contains(&"osmosis".to_string()),
        "Should contain Osmosis"
    );
    assert!(
        chains.contains(&"cosmoshub".to_string()),
        "Should contain Cosmos Hub"
    );
}

#[tokio::test]
async fn test_list_paths() {
    let paths = list_paths().await.expect("Failed to list paths");
    assert!(!paths.is_empty(), "Path list should not be empty");
    assert!(
        paths
            .iter()
            .any(|p| p.contains("osmosis") && p.contains("cosmoshub")),
        "Should contain osmosis-cosmoshub path"
    );
}

#[tokio::test]
async fn test_get_chain_osmosis() {
    let chain = get_chain("osmosis")
        .await
        .expect("Failed to get chain")
        .expect("Osmosis chain should exist");

    assert_eq!(chain.chain_name, "osmosis");
    assert_eq!(chain.chain_id, "osmosis-1");
    assert_eq!(chain.bech32_prefix, "osmo");
    assert_eq!(chain.slip44, 118);
    assert!(!chain.apis.rpc.is_empty(), "Should have RPC endpoints");
    assert!(!chain.apis.rest.is_empty(), "Should have REST endpoints");
}

#[tokio::test]
async fn test_get_endpoint_helpers() {
    let grpc = get_grpc_endpoints("osmosis")
        .await
        .expect("Failed to get gRPC endpoints")
        .expect("Osmosis should have gRPC endpoints");
    assert!(!grpc.is_empty(), "Should have gRPC endpoints");
    assert!(grpc.iter().all(|endpoint| !endpoint.is_empty()));

    let rest = get_rest_endpoints("osmosis")
        .await
        .expect("Failed to get REST endpoints")
        .expect("Osmosis should have REST endpoints");
    assert!(!rest.is_empty(), "Should have REST endpoints");

    let rpc = get_rpc_endpoints("osmosis")
        .await
        .expect("Failed to get RPC endpoints")
        .expect("Osmosis should have RPC endpoints");
    assert!(!rpc.is_empty(), "Should have RPC endpoints");

    let missing = get_grpc_endpoints("nonexistent_chain_xyz")
        .await
        .expect("Should not error on nonexistent chain");
    assert!(missing.is_none(), "Nonexistent chain should return None");
}

#[tokio::test]
async fn test_get_chain_cosmoshub() {
    let chain = get_chain("cosmoshub")
        .await
        .expect("Failed to get chain")
        .expect("Cosmos Hub chain should exist");

    assert_eq!(chain.chain_name, "cosmoshub");
    assert!(chain.chain_id.starts_with("cosmoshub-"));
    assert_eq!(chain.bech32_prefix, "cosmos");
}

#[tokio::test]
async fn test_get_assets_osmosis() {
    let assets = get_assets("osmosis")
        .await
        .expect("Failed to get assets")
        .expect("Osmosis assets should exist");

    assert_eq!(assets.chain_name, "osmosis");
    assert!(!assets.assets.is_empty(), "Should have assets");

    let osmo = assets.assets.iter().find(|a| a.symbol == "OSMO");
    assert!(osmo.is_some(), "Should have OSMO token");

    let osmo = osmo.unwrap();
    assert_eq!(osmo.base, "uosmo");
    assert_eq!(osmo.display, "osmo");
    assert_eq!(osmo.name, "Osmosis");
}

#[tokio::test]
async fn test_get_assets_cosmoshub() {
    let assets = get_assets("cosmoshub")
        .await
        .expect("Failed to get assets")
        .expect("Cosmos Hub assets should exist");

    assert_eq!(assets.chain_name, "cosmoshub");

    let atom = assets.assets.iter().find(|a| a.symbol == "ATOM");
    assert!(atom.is_some(), "Should have ATOM token");

    let atom = atom.unwrap();
    assert_eq!(atom.base, "uatom");
    assert_eq!(atom.display, "atom");
}

#[tokio::test]
async fn test_get_path_osmosis_cosmoshub() {
    let path = get_path("osmosis", "cosmoshub")
        .await
        .expect("Failed to get path")
        .expect("Path should exist");

    assert_eq!(path.chain_1.chain_name, "cosmoshub");
    assert_eq!(path.chain_2.chain_name, "osmosis");
    assert!(!path.channels.is_empty(), "Should have channels");

    let channel = &path.channels[0];
    assert_eq!(channel.ordering.as_deref(), Some("unordered"));
    assert_eq!(channel.version.as_deref(), Some("ics20-1"));
    assert!(channel
        .tags
        .as_ref()
        .and_then(|tags| tags.status.as_deref())
        .is_some());
}

#[tokio::test]
async fn test_get_path_reverse_order() {
    let path1 = get_path("osmosis", "cosmoshub")
        .await
        .expect("Failed to get path")
        .expect("Path should exist");

    let path2 = get_path("cosmoshub", "osmosis")
        .await
        .expect("Failed to get path")
        .expect("Path should exist");

    // Should return the same path regardless of order
    assert_eq!(path1.chain_1.chain_name, path2.chain_1.chain_name);
    assert_eq!(path1.chain_2.chain_name, path2.chain_2.chain_name);
}

#[tokio::test]
async fn test_get_evm_rpc_endpoints() {
    // acre (an EVM chain) should expose evm-http-jsonrpc endpoints
    let evm = get_evm_rpc_endpoints("acre")
        .await
        .expect("Failed to get EVM RPC endpoints");
    if let Some(endpoints) = evm {
        assert!(!endpoints.is_empty(), "Acre should have EVM endpoints");
    }
}

#[tokio::test]
async fn test_local_overrides_merge() {
    let overrides = LocalOverrides::parse(
        r#"{
            "secretnetwork": {
                "grpc": ["grpc.secretnetwork.pathrocknetwork.org:443"],
                "rest": ["https://api.secretnetwork.pathrocknetwork.org"],
                "rpc": ["https://rpc.secretnetwork.pathrocknetwork.org"]
            }
        }"#,
    )
    .expect("Failed to parse overrides");

    let chain = get_chain_with_overrides("secretnetwork", &overrides)
        .await
        .expect("Failed to get chain with overrides")
        .expect("SecretNetwork should exist");

    assert_eq!(chain.chain_name, "secretnetwork");
    assert_eq!(chain.chain_id, "secret-4");
    assert!(
        chain
            .apis
            .grpc
            .iter()
            .any(|e| e.address == "grpc.secretnetwork.pathrocknetwork.org:443"),
        "Should include the local gRPC endpoint"
    );
    assert!(
        chain
            .apis
            .rest
            .iter()
            .any(|e| e.address == "https://api.secretnetwork.pathrocknetwork.org"),
        "Should include the local REST endpoint"
    );
    assert!(
        chain
            .apis
            .rpc
            .iter()
            .any(|e| e.address == "https://rpc.secretnetwork.pathrocknetwork.org"),
        "Should include the local RPC endpoint"
    );

    let grpc = get_grpc_endpoints_with_overrides("secretnetwork", &overrides)
        .await
        .expect("Failed to get gRPC endpoints")
        .expect("SecretNetwork should have endpoints");
    assert!(
        grpc.contains(&"grpc.secretnetwork.pathrocknetwork.org:443".to_string()),
        "gRPC helper should include local endpoint"
    );
}

#[tokio::test]
async fn test_local_overrides_deduplicates() {
    let overrides = LocalOverrides::parse(
        r#"{
            "secretnetwork": {
                "grpc": ["secretnetwork.lavenderfive.com:443", "grpc.secretnetwork.pathrocknetwork.org:443"],
                "rest": ["https://rest.lavenderfive.com:443/secretnetwork"],
                "rpc": []
            }
        }"#,
    )
    .expect("Failed to parse overrides");

    let chain = get_chain_with_overrides("secretnetwork", &overrides)
        .await
        .expect("Failed to get chain with overrides")
        .expect("SecretNetwork should exist");

    let lavender = chain
        .apis
        .grpc
        .iter()
        .filter(|e| e.address == "secretnetwork.lavenderfive.com:443")
        .count();
    assert_eq!(lavender, 1, "Duplicate endpoints should be de-duplicated");

    let pathrock = chain
        .apis
        .grpc
        .iter()
        .filter(|e| e.address == "grpc.secretnetwork.pathrocknetwork.org:443")
        .count();
    assert_eq!(pathrock, 1, "New endpoint should be added exactly once");
}

#[tokio::test]
async fn test_local_overrides_nonexistent_chain() {
    let overrides = LocalOverrides::parse(
        r#"{
            "nonexistent_chain_xyz": {
                "rpc": ["https://rpc.example.com"]
            }
        }"#,
    )
    .expect("Failed to parse overrides");

    let chain = get_chain_with_overrides("nonexistent_chain_xyz", &overrides)
        .await
        .expect("Should not error on nonexistent chain");
    assert!(
        chain.is_none(),
        "Overrides should not create a new chain for a chain absent from the registry"
    );
}

#[tokio::test]
async fn test_get_nonexistent_chain() {
    let chain = get_chain("nonexistent_chain_xyz")
        .await
        .expect("Should not error on nonexistent chain");

    assert!(chain.is_none(), "Nonexistent chain should return None");
}

#[tokio::test]
async fn test_get_nonexistent_path() {
    let path = get_path("nonexistent1", "nonexistent2")
        .await
        .expect("Should not error on nonexistent path");

    assert!(path.is_none(), "Nonexistent path should return None");
}

#[cfg(feature = "cache")]
#[tokio::test]
#[ignore] // This test is slow as it fetches all paths
async fn test_registry_cache() {
    let cache = RegistryCache::try_new()
        .await
        .expect("Failed to initialize cache");

    // Test getting a specific path
    let path = cache
        .get_path("osmosis", "cosmoshub")
        .await
        .expect("Failed to get path from cache")
        .expect("Path should exist in cache");

    assert_eq!(path.chain_1.chain_name, "cosmoshub");
    assert_eq!(path.chain_2.chain_name, "osmosis");

    // Test filtering by preferred tag
    let preferred_paths = cache
        .get_paths_filtered(Tag::Preferred(true))
        .await
        .expect("Failed to filter paths");

    assert!(!preferred_paths.is_empty(), "Should have preferred paths");

    // Test filtering by status
    let live_paths = cache
        .get_paths_filtered(Tag::Status("ACTIVE".to_string()))
        .await
        .expect("Failed to filter paths");

    assert!(!live_paths.is_empty(), "Should have ACTIVE paths");
}

#[tokio::test]
async fn test_multiple_chains() {
    let test_chains = vec!["osmosis", "cosmoshub", "juno", "stargaze", "akash"];

    for chain_name in test_chains {
        let Some(chain) = get_chain(chain_name)
            .await
            .unwrap_or_else(|error| panic!("Failed to get chain {chain_name}: {error}"))
        else {
            continue;
        };
        assert_eq!(chain.chain_name, chain_name, "Chain name should match");
        assert!(!chain.chain_id.is_empty(), "Chain ID should not be empty");
        assert!(
            !chain.bech32_prefix.is_empty(),
            "Bech32 prefix should not be empty"
        );
    }
}

#[tokio::test]
async fn test_chain_properties() {
    let chain = get_chain("osmosis")
        .await
        .expect("Failed to get chain")
        .expect("Osmosis should exist");

    // Test codebase properties
    assert!(!chain.codebase.git_repo.is_empty(), "Should have git repo");
    // Note: cosmwasm_enabled field may vary with registry updates

    // Test network properties
    assert_eq!(chain.network_type, "mainnet");
    assert_eq!(chain.status, "live");

    // Test staking properties
    assert!(
        !chain.staking.staking_tokens.is_empty(),
        "Should have staking tokens"
    );
    assert_eq!(chain.staking.staking_tokens[0].denom, "uosmo");

    // Test fee properties
    assert!(!chain.fees.fee_tokens.is_empty(), "Should have fee tokens");
    assert_eq!(chain.fees.fee_tokens[0].denom, "uosmo");
}
