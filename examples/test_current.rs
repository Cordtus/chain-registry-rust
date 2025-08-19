use chain_registry::get::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing chain-registry crate with current registry...\n");
    
    // Test listing chains
    println!("Fetching chain list...");
    match list_chains().await {
        Ok(chains) => {
            println!("✓ Found {} chains", chains.len());
            println!("  Sample chains: {:?}", &chains[..5.min(chains.len())]);
        }
        Err(e) => {
            println!("✗ Failed to list chains: {}", e);
            return Err(e.into());
        }
    }
    
    // Test getting specific chain info
    println!("\nFetching Osmosis chain info...");
    match get_chain("osmosis").await {
        Ok(Some(chain)) => {
            println!("✓ Got Osmosis chain info");
            println!("  Chain ID: {}", chain.chain_id);
            println!("  Chain Name: {}", chain.chain_name);
            println!("  Bech32 Prefix: {}", chain.bech32_prefix);
        }
        Ok(None) => {
            println!("✗ Osmosis chain not found");
            return Err("Osmosis chain not found".into());
        }
        Err(e) => {
            println!("✗ Failed to get chain: {}", e);
            return Err(e.into());
        }
    }
    
    // Test getting assets
    println!("\nFetching Osmosis assets...");
    match get_assets("osmosis").await {
        Ok(Some(assets)) => {
            println!("✓ Got {} assets for Osmosis", assets.assets.len());
            if !assets.assets.is_empty() {
                println!("  First asset: {}", assets.assets[0].name);
            }
        }
        Ok(None) => {
            println!("✗ Osmosis assets not found");
            return Err("Osmosis assets not found".into());
        }
        Err(e) => {
            println!("✗ Failed to get assets: {}", e);
            return Err(e.into());
        }
    }
    
    // Test getting IBC paths
    println!("\nFetching IBC path between Osmosis and Cosmos Hub...");
    match get_path("osmosis", "cosmoshub").await {
        Ok(Some(path)) => {
            println!("✓ Got IBC path");
            println!("  Chain 1: {}", path.chain_1.chain_name);
            println!("  Chain 2: {}", path.chain_2.chain_name);
            println!("  Channel: {}-{}", path.channels[0].chain_1.channel_id, path.channels[0].chain_2.channel_id);
        }
        Ok(None) => {
            println!("✗ IBC path not found");
            return Err("IBC path not found".into());
        }
        Err(e) => {
            println!("✗ Failed to get path: {}", e);
            return Err(e.into());
        }
    }
    
    println!("\n✓ All tests passed!");
    Ok(())
}