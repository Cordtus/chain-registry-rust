#![cfg(feature = "cache")]
#![cfg_attr(docsrs, doc(cfg(feature = "cache")))]
use assay::assay;
use cosmos_chain_registry::{cache::RegistryCache, paths::Tag};

#[assay]
async fn registry_cache_happy_path() {
    let cache = RegistryCache::try_new()
        .await
        .expect("failed to initialize cache");
    let chain_a = "cosmoshub";
    let chain_b = "osmosis";
    let result = cache.get_path(chain_a, chain_b).await.unwrap().unwrap();
    assert_eq!(result.chain_1.chain_name, "cosmoshub");
    assert_eq!(result.chain_2.chain_name, "osmosis");

    // reverse order
    let result = cache.get_path(chain_b, chain_a).await.unwrap().unwrap();
    assert_eq!(result.chain_1.chain_name, "cosmoshub");
    assert_eq!(result.chain_2.chain_name, "osmosis");

    // filter by tag values known to be present in the current registry
    let preferred = true;
    let result = cache
        .get_paths_filtered(Tag::Preferred(preferred))
        .await
        .unwrap();
    assert!(!result.is_empty());
    result.iter().for_each(|r| {
        assert!(r
            .channels
            .iter()
            .any(|chan| chan.tags.as_ref().map(|t| t.preferred) == Some(preferred)));
    });

    let status = "ACTIVE".to_string();
    let result = cache
        .get_paths_filtered(Tag::Status(status.clone()))
        .await
        .unwrap();
    assert!(!result.is_empty());
    result.iter().for_each(|r| {
        assert!(r.channels.iter().any(|chan| {
            chan.tags.as_ref().and_then(|t| t.status.as_deref()) == Some(status.as_str())
        }));
    });
}
