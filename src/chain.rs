#![allow(clippy::derive_partial_eq_without_eq)]
/// Contains models for serializing and deserializing the `chain.json` in a given chain's directory in the registry repository
use crate::assets::{Image, LogoURIs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// by denying unknown fields we can be more confident that our structs match the
// current configured GIT_REF's schema. errors will occur if the chain.json is
// formatted incorrectly, however.
#[serde(default)]
pub struct ChainInfo {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub chain_name: String,
    pub chain_type: String,
    pub chain_id: String,
    pub pre_fork_chain_name: String,
    pub pretty_name: String,
    pub website: String,
    pub status: String,
    pub network_type: String,
    pub bech32_prefix: String,
    pub bech32_config: Bech32Config,
    pub daemon_name: String,
    pub node_home: String,
    pub key_algos: Vec<String>,
    pub slip44: u32,
    pub alternative_slip44s: Vec<u32>,
    pub fees: Fees,
    pub staking: Staking,
    pub codebase: Codebase,
    pub images: Vec<Image>,
    pub logo_uris: LogoURIs,
    pub description: String,
    pub peers: Peers,
    pub apis: Apis,
    pub snapshots: Vec<Snapshot>,
    pub explorers: Vec<Explorer>,
    pub keywords: Vec<String>,
    pub extra_codecs: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Bech32Config {
    #[serde(rename = "bech32PrefixAccAddr")]
    pub acc_addr: String,
    #[serde(rename = "bech32PrefixAccPub")]
    pub acc_pub: String,
    #[serde(rename = "bech32PrefixConsAddr")]
    pub cons_addr: String,
    #[serde(rename = "bech32PrefixConsPub")]
    pub cons_pub: String,
    #[serde(rename = "bech32PrefixValAddr")]
    pub val_addr: String,
    #[serde(rename = "bech32PrefixValPub")]
    pub val_pub: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Snapshot {
    pub provider: String,
    pub url: String,
    pub latest_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub db_backend: String,
    pub frequency: String,
    pub compression: String,
    pub checksum_available: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Codebase {
    pub git_repo: String,
    pub recommended_version: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub compatible_versions: Vec<String>,
    pub tag: String,
    pub language: Option<Language>,
    pub binaries: Binaries,
    pub sdk: Option<Sdk>,
    pub consensus: Option<Consensus>,
    pub cosmwasm: Option<Cosmwasm>,
    pub ibc: Option<Ibc>,
    pub genesis: Option<Genesis>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Genesis {
    pub name: String,
    pub genesis_url: String,
    pub ics_ccv_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Sdk {
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
    pub repo: String,
    pub tag: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Consensus {
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
    pub repo: String,
    pub tag: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Language {
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
    pub repo: String,
    pub tag: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Cosmwasm {
    pub version: String,
    pub repo: String,
    pub tag: String,
    pub enabled: Option<bool>,
    pub path: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Ibc {
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
    pub repo: String,
    pub tag: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub ics_enabled: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Binaries {
    #[serde(rename = "linux/amd64")]
    pub linux_amd_64: String,
    #[serde(rename = "linux/arm64")]
    pub linux_arm_64: String,
    #[serde(rename = "darwin/amd64")]
    pub darwin_amd_64: String,
    #[serde(rename = "darwin/arm64")]
    pub darwin_arm_64: String,
    #[serde(rename = "windows/amd64")]
    pub windows_amd_64: String,
    #[serde(rename = "windows/arm64")]
    pub windows_arm_64: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Peers {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub seeds: Vec<Seed>,
    pub persistent_peers: Vec<PersistentPeer>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Seed {
    pub id: String,
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PersistentPeer {
    pub id: String,
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Apis {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rpc: Vec<Rpc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rest: Vec<Rest>,
    pub grpc: Vec<Grpc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Rpc {
    pub address: String,
    pub provider: Option<String>,
    pub archive: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Rest {
    pub address: String,
    pub provider: Option<String>,
    pub archive: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Grpc {
    pub address: String,
    pub provider: Option<String>,
    pub archive: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Fees {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub fee_tokens: Vec<FeeToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct FeeToken {
    pub denom: String,
    pub fixed_min_gas_price: f32,
    pub low_gas_price: f32,
    pub average_gas_price: f32,
    pub high_gas_price: f32,
    pub gas_costs: GasCosts,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct GasCosts {
    pub cosmos_send: i64,
    pub ibc_transfer: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Staking {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub staking_tokens: Vec<StakingToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct StakingToken {
    pub denom: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Explorer {
    pub kind: String,
    pub url: String,
    pub tx_page: String,
    pub account_page: String,
    pub validator_page: String,
    pub proposal_page: String,
    pub block_page: String,
}
