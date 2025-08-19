// Example showing how this crate could be used for a Telegram bot
// similar to cosmoclerk but without needing to clone the entire registry

use chain_registry::get::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Simple in-memory cache with TTL
struct RegistryCache {
    chains: HashMap<String, (ChainInfo, Instant)>,
    assets: HashMap<String, (AssetList, Instant)>,
    paths: HashMap<String, (IBCPath, Instant)>,
    ttl: Duration,
}

impl RegistryCache {
    fn new(ttl_minutes: u64) -> Self {
        Self {
            chains: HashMap::new(),
            assets: HashMap::new(),
            paths: HashMap::new(),
            ttl: Duration::from_secs(ttl_minutes * 60),
        }
    }

    async fn get_chain(&mut self, name: &str) -> Result<Option<ChainInfo>, Box<dyn std::error::Error>> {
        // Check cache first
        if let Some((chain, timestamp)) = self.chains.get(name) {
            if timestamp.elapsed() < self.ttl {
                return Ok(Some(chain.clone()));
            }
        }

        // Fetch from registry
        let chain = get_chain(name).await?;
        if let Some(ref c) = chain {
            self.chains.insert(name.to_string(), (c.clone(), Instant::now()));
        }
        Ok(chain)
    }

    async fn get_assets(&mut self, name: &str) -> Result<Option<AssetList>, Box<dyn std::error::Error>> {
        // Check cache first
        if let Some((assets, timestamp)) = self.assets.get(name) {
            if timestamp.elapsed() < self.ttl {
                return Ok(Some(assets.clone()));
            }
        }

        // Fetch from registry
        let assets = get_assets(name).await?;
        if let Some(ref a) = assets {
            self.assets.insert(name.to_string(), (a.clone(), Instant::now()));
        }
        Ok(assets)
    }

    async fn get_path(&mut self, chain_a: &str, chain_b: &str) -> Result<Option<IBCPath>, Box<dyn std::error::Error>> {
        let key = format!("{}-{}", chain_a.min(chain_b), chain_a.max(chain_b));
        
        // Check cache first
        if let Some((path, timestamp)) = self.paths.get(&key) {
            if timestamp.elapsed() < self.ttl {
                return Ok(Some(path.clone()));
            }
        }

        // Fetch from registry
        let path = get_path(chain_a, chain_b).await?;
        if let Some(ref p) = path {
            self.paths.insert(key, (p.clone(), Instant::now()));
        }
        Ok(path)
    }
}

// Simulated bot commands
async fn handle_chain_info(cache: &mut RegistryCache, chain_name: &str) -> String {
    match cache.get_chain(chain_name).await {
        Ok(Some(chain)) => {
            format!(
                "🔗 **{}**\n\
                Chain ID: `{}`\n\
                Prefix: `{}`\n\
                Status: {}\n\
                Type: {}\n\
                {} RPC endpoints available\n\
                {} REST endpoints available",
                chain.pretty_name,
                chain.chain_id,
                chain.bech32_prefix,
                chain.status,
                chain.network_type,
                chain.apis.rpc.len(),
                chain.apis.rest.len()
            )
        }
        Ok(None) => format!("❌ Chain '{}' not found in registry", chain_name),
        Err(e) => format!("❌ Error fetching chain info: {}", e),
    }
}

async fn handle_assets_info(cache: &mut RegistryCache, chain_name: &str) -> String {
    match cache.get_assets(chain_name).await {
        Ok(Some(assets)) => {
            let mut response = format!("💰 **Assets on {}**\n\n", chain_name);
            for asset in assets.assets.iter().take(5) {
                response.push_str(&format!(
                    "• {} ({}) - {}\n",
                    asset.symbol,
                    asset.name,
                    asset.display
                ));
            }
            if assets.assets.len() > 5 {
                response.push_str(&format!("\n... and {} more", assets.assets.len() - 5));
            }
            response
        }
        Ok(None) => format!("❌ Assets for '{}' not found in registry", chain_name),
        Err(e) => format!("❌ Error fetching assets: {}", e),
    }
}

async fn handle_ibc_info(cache: &mut RegistryCache, chain_a: &str, chain_b: &str) -> String {
    match cache.get_path(chain_a, chain_b).await {
        Ok(Some(path)) => {
            let channel = &path.channels[0];
            format!(
                "🌉 **IBC Path**\n\
                {} ↔️ {}\n\n\
                Channel: `{}`-`{}`\n\
                Port: `{}`\n\
                Status: {}\n\
                Version: {}",
                path.chain_1.chain_name,
                path.chain_2.chain_name,
                channel.chain_1.channel_id,
                channel.chain_2.channel_id,
                channel.chain_1.port_id,
                channel.tags.status,
                channel.version
            )
        }
        Ok(None) => format!("❌ No IBC path found between '{}' and '{}'", chain_a, chain_b),
        Err(e) => format!("❌ Error fetching IBC path: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Chain Registry Bot Concept\n");
    println!("This example demonstrates how to use the chain-registry crate");
    println!("for a Telegram bot without cloning the entire repository.\n");

    // Create cache with 30-minute TTL
    let mut cache = RegistryCache::new(30);

    // Simulate bot commands
    println!("📡 Simulating /chain osmosis command:");
    println!("{}\n", handle_chain_info(&mut cache, "osmosis").await);

    println!("📡 Simulating /assets osmosis command:");
    println!("{}\n", handle_assets_info(&mut cache, "osmosis").await);

    println!("📡 Simulating /ibc osmosis cosmoshub command:");
    println!("{}\n", handle_ibc_info(&mut cache, "osmosis", "cosmoshub").await);

    // Demonstrate cache hit
    println!("📡 Simulating /chain osmosis command again (from cache):");
    let start = Instant::now();
    println!("{}", handle_chain_info(&mut cache, "osmosis").await);
    println!("⚡ Response time: {:?} (cached)\n", start.elapsed());

    // List available chains
    println!("📡 Available chains:");
    let chains = list_chains().await?;
    println!("Found {} chains total", chains.len());
    println!("Sample: {:?}", &chains[..10.min(chains.len())]);

    Ok(())
}