#![cfg_attr(docsrs, doc(cfg(feature = "cache")))]
/// Provides caching of registry data for easy querying and filtering. It's recommended to populate the cache during the startup
/// for a long-running process as construction involves sending an individual GET request for every path in the registry which
/// takes a while.
use crate::{
    get::*,
    paths::{IBCPath, Tag},
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

// TO-DO:
// - Option to load from local repo clone
// - Currently don't see a need to cache chain/asset info but might need it in the future
/// Used to cache IBC path data from the chain registry for easy filtering.
#[derive(Default, Deserialize, Serialize)]
pub struct RegistryCache {
    paths: HashMap<String, IBCPath>,
}

impl RegistryCache {
    /// Returns a cached [`IBCPath`] representing a channel between `chain_a` and `chain_b` if it exists.
    /// Passing in the same value for `chain_a` and `chain_b` will always return `Ok(None)`.
    ///
    /// # Arguments
    ///
    /// * `chain_a` - A chain name. Must match a directory name in the root of the chain registry repository `<https://github.com/cosmos/chain-registry>`
    /// * `chain_b` - A chain name. Must match a directory name in the root of the chain registry repository `<https://github.com/cosmos/chain-registry>`
    pub async fn get_path(&self, chain_a: &str, chain_b: &str) -> Result<Option<IBCPath>> {
        let path_name = match chain_a.cmp(chain_b) {
            Ordering::Less => chain_a.to_string() + "-" + chain_b,
            Ordering::Equal => return Ok(None),
            Ordering::Greater => chain_b.to_string() + "-" + chain_a,
        };

        Ok(self.paths.get(&path_name).cloned())
    }

    /// Returns all cached [`IBCPath`]s involving a specific chain
    pub async fn get_paths_for_chain(&self, chain_name: &str) -> Result<Vec<IBCPath>> {
        Ok(self
            .paths
            .iter()
            .filter(|(_, path)| {
                path.chain_1.chain_name == chain_name || path.chain_2.chain_name == chain_name
            })
            .map(|(_, path)| path.to_owned())
            .collect())
    }

    /// Returns all cached [`IBCPath`]s
    pub fn get_all_paths(&self) -> Vec<IBCPath> {
        self.paths.values().cloned().collect()
    }

    /// Returns paths containing a specific channel ID
    pub async fn get_paths_by_channel(&self, channel_id: &str) -> Result<Vec<IBCPath>> {
        Ok(self
            .paths
            .iter()
            .filter(|(_, path)| {
                path.channels.iter().any(|chan| {
                    chan.chain_1.channel_id == channel_id || chan.chain_2.channel_id == channel_id
                })
            })
            .map(|(_, path)| path.to_owned())
            .collect())
    }

    /// Returns paths with a specific client ID
    pub async fn get_paths_by_client(&self, client_id: &str) -> Result<Vec<IBCPath>> {
        Ok(self
            .paths
            .iter()
            .filter(|(_, path)| {
                path.chain_1.client_id == client_id || path.chain_2.client_id == client_id
            })
            .map(|(_, path)| path.to_owned())
            .collect())
    }

    /// Returns cached [`IBCPath`] that match a provided [`Tag`]
    ///
    /// # Arguments
    ///
    /// * `tag` - A [`Tag`] representing the the desired key/value pair to filter by.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use chain_registry::cache::{RegistryCache, Tag};
    ///
    /// // store paths from the registry repository in a cache
    /// let cache = RegistryCache::try_new().await?;
    /// let dex = "osmosis".to_string();
    ///
    /// // paths will contain a vec of all IBC paths containing the tag dex:osmosis
    /// let paths = cache.get_paths_filtered(Tag::Dex(dex))?;
    /// ```
    pub async fn get_paths_filtered(&self, tag: Tag) -> Result<Vec<IBCPath>> {
        Ok(self
            .paths
            .iter()
            .filter(|kv| {
                kv.1.channels.iter().any(|chan| {
                    if let Some(tags) = &chan.tags {
                        match &tag {
                            Tag::Dex(d) => tags.dex.as_ref().is_some_and(|dex| dex.eq(d)),
                            Tag::Preferred(p) => tags.preferred.eq(p),
                            Tag::Properties(p) => tags.properties.as_ref().is_some_and(|props| props.eq(p)),
                            Tag::Status(s) => tags.status.as_ref().is_some_and(|status| status.eq(s)),
                        }
                    } else {
                        false
                    }
                })
            })
            .map(|kv| kv.1.to_owned())
            .collect())
    }

    /// Creates a new cache by retrieving and deserializing each [`IBCPath`] from the Cosmos Chain Registry for easy filtering
    pub async fn try_new() -> Result<RegistryCache> {
        let path_names = list_paths().await?;
        let mut paths = HashMap::<String, IBCPath>::default();

        for pn in path_names {
            let cn: Vec<&str> = pn.split('-').collect();

            // this unwrap is safe becauase we query the path directly from the list of path .json file names
            // retrieved earlier, therefore the Option returned should never be None.
            paths.insert(
                pn.clone(),
                get_path(cn[0], cn[1]).await?.expect("path returned None"),
            );
        }

        Ok(RegistryCache { paths })
    }
}
