# cosmos-chain-registry

A Rust API for interacting with the [Cosmos chain registry repository](https://github.com/cosmos/chain-registry)

## Why This Fork?

This crate is a maintained fork of the original [chain-registry](https://crates.io/crates/chain-registry) crate. We created this because:

1. **Abandoned Original**: The original crate hasn't been updated since September 2022 (2+ years ago)
2. **Security Vulnerabilities**: The original has 14+ critical security vulnerabilities in its dependencies
3. **Outdated Data**: The original uses a fixed 2022 commit, while this fork always fetches the latest chain-registry data
4. **Better Error Handling**: This fork returns `Option::None` for missing chains/paths instead of errors
5. **Active Maintenance**: Regular updates with the latest Cosmos ecosystem changes

## Features

- **Complete JSON Support**: Models for serializing/deserializing chain.json, assetlist.json, and IBC path JSON files
- **Tolerant Deserialization**: Gracefully handles missing or unrecognized JSON fields, ensuring compatibility even as the registry evolves
- **Simple API**: High-level get/list methods for retrieving chain, asset, and path data
- **Advanced IBC Path Cache**: Comprehensive filtering and query capabilities for IBC paths:
  - Filter by chain, channel ID, client ID, or custom tags
  - Query paths between specific chains
  - Efficient in-memory caching for long-running processes
- **Always Current**: Automatically fetches the latest chain-registry data from the master branch
- **Comprehensive Error Handling**: Uses `eyre` for detailed error context
- **Full Test Coverage**: Thoroughly tested against the live registry

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cosmos-chain-registry = "0.4.0"
```

## Usage

### Basic Usage

```rust
use chain_registry::get::{get_chain, get_assets, get_path, list_chains};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Get chain information
    let chain = get_chain("osmosis").await?.unwrap();
    println!("Chain ID: {}", chain.chain_id);

    // Get asset information
    let assets = get_assets("osmosis").await?.unwrap();
    println!("Found {} assets", assets.assets.len());

    // Get IBC path information
    let path = get_path("osmosis", "cosmoshub").await?.unwrap();
    println!("IBC Path: {}-{}", path.chain_1.chain_name, path.chain_2.chain_name);

    // List all available chains
    let chains = list_chains().await?;
    println!("Found {} chains in the registry", chains.len());

    Ok(())
}
```

### Using the IBC Path Cache

```rust
use chain_registry::{cache::RegistryCache, paths::Tag};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Build the cache (fetches all IBC paths - may take a moment)
    let cache = RegistryCache::try_new().await?;

    // Get path between specific chains
    let path = cache.get_path("osmosis", "cosmoshub").await?.unwrap();

    // Get all paths for a specific chain
    let osmosis_paths = cache.get_paths_for_chain("osmosis").await?;
    println!("Osmosis has {} IBC connections", osmosis_paths.len());

    // Filter paths by channel ID
    let channel_paths = cache.get_paths_by_channel("channel-141").await?;

    // Filter by custom tags
    let dex_paths = cache.get_paths_filtered(Tag::Dex("osmosis".to_string())).await?;

    Ok(())
}
```

## Warning

The chain registry is unversioned and syntax is unenforced. This library is written to ignore unrecognized or missing JSON fields but it isn't guaranteed to work for all registry items.

## Migration from `chain-registry`

To migrate from the original crate:

1. Update your `Cargo.toml`: Replace `chain-registry` with `cosmos-chain-registry`
2. Update your imports: Replace `use chain_registry::*` with `use cosmos_chain_registry::*`
3. Handle `None` returns: Methods now return `Option<T>` for missing data instead of errors

## License

Apache-2.0

## Credits

Originally created by Collin Brittain. This fork is maintained by Cosmos Labs with contributions from the community.