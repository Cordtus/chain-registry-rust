//! An API for the [Cosmos chain registry](http://github.com/cosmos/chain-registry)
//!
//! A more current, updated fork of the original `chain-registry` crate with security updates,
//! bug fixes, and always-current chain data.
//!
//! # Features
//!
//! * Support for chain.json, assetlist.json, and IBC path JSON files
//! * Tolerant deserialization that gracefully handles missing or unrecognized fields
//! * Cache type with comprehensive filtering options for IBC paths
//! * Simple get/list methods for chains, assets, and paths
//! * Always fetches from master branch for latest registry data
//!
//! # Quick Start
//!
//! ```no_run
//! use chain_registry::get::{get_chain, get_assets, get_path, list_chains};
//!
//! #[tokio::main]
//! async fn main() -> eyre::Result<()> {
//!     // Get chain information
//!     let chain_info = get_chain("cosmoshub").await?.unwrap();
//!
//!     // Get asset information
//!     let assets = get_assets("osmosis").await?.unwrap();
//!
//!     // Get IBC path between two chains
//!     let path = get_path("cosmoshub", "osmosis").await?.unwrap();
//!
//!     // List all chains
//!     let chains = list_chains().await?;
//!
//!     Ok(())
//! }
//! ```

/// Models for assets.json ser/de
pub mod assets;

/// Models for chain.json ser/de
pub mod chain;

/// A cache type for reading IBC path data into memory for faster and filterable queries
#[cfg(feature = "cache")]
pub mod cache;

/// API for getting and listing data from the registry Github repo
pub mod get;
pub mod github;

/// Models for IBC path JSON ser/de
pub mod paths;

// Re-export commonly used types at crate root
pub use assets::AssetList;
pub use chain::ChainInfo;
pub use paths::IBCPath;

#[cfg(feature = "cache")]
pub use cache::RegistryCache;
