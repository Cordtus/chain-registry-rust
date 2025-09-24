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

- Models for serializing and deserializing chain.json, assets.json and IBC path JSON files
- Simple get/list methods for retrieving chain, asset, and path data
- A cache type (currently only supports IBC Path data) that exposes additional filtering options
- Always fetches the latest chain-registry data from the master branch
- Comprehensive error handling using `eyre`
- Full test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cosmos-chain-registry = "0.3.0"
```

## Usage

```rust
use cosmos_chain_registry::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get chain information
    let chain = get_chain("osmosis").await?.unwrap();
    println!("Chain ID: {}", chain.chain_id);

    // Get asset information
    let assets = get_assets("osmosis").await?.unwrap();
    println!("Found {} assets", assets.assets.len());

    // Get IBC path information
    let path = get_path("osmosis", "cosmoshub").await?.unwrap();
    println!("IBC Path: {}-{}", path.chain_1.chain_name, path.chain_2.chain_name);

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