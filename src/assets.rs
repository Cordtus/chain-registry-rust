/// Contains models for serializing and deserializing `assets.json` for a given chain
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct AssetList {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub chain_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Asset {
    pub deprecated: Option<bool>,
    pub description: String,
    pub extended_description: String,
    pub denom_units: Vec<DenomUnit>,
    #[serde(rename = "type_asset")]
    pub type_asset: String,
    pub address: String,
    pub base: String,
    pub name: String,
    pub display: String,
    pub symbol: String,
    pub traces: Vec<Trace>,
    pub ibc: Option<IbcInfo>,
    #[serde(rename = "logo_URIs")]
    pub logo_uris: LogoURIs,
    pub images: Vec<Image>,
    pub coingecko_id: String,
    pub keywords: Vec<String>,
    pub socials: Socials,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: i32,
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct LogoURIs {
    pub png: String,
    pub svg: String,
}

/// A pointer to another registry asset, used to express provenance and image sync
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Pointer {
    pub chain_name: String,
    pub base_denom: String,
}

/// An asset image. `image_sync` and `theme` are optional.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Image {
    pub image_sync: Option<Pointer>,
    pub png: String,
    pub svg: String,
    pub theme: Option<ImageTheme>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct ImageTheme {
    pub circle: Option<bool>,
    pub dark_mode: Option<bool>,
}

/// A transition describing how an asset was minted, bridged, or wrapped
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Trace {
    #[serde(rename = "type")]
    pub r#type: String,
    pub counterparty: Option<Counterparty>,
    pub chain: Option<TraceChain>,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Counterparty {
    pub chain_name: String,
    pub base_denom: String,
    pub channel_id: Option<String>,
    pub port: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct TraceChain {
    pub channel_id: Option<String>,
    pub path: Option<String>,
    pub port: Option<String>,
}

/// Legacy IBC channel metadata between source and destination chain
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct IbcInfo {
    pub source_channel: String,
    pub dst_channel: String,
    pub source_denom: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Socials {
    pub website: String,
    pub x: String,
    pub telegram: String,
    pub discord: String,
    pub github: String,
    pub medium: String,
    pub reddit: String,
}
