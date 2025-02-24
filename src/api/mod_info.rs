use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Struct representing the information of a mod. that's given in the modinfo.json file.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
#[derive(Clone)]
pub struct ModInfo {
    /// The type of the mod.
    #[serde(rename = "type", deserialize_with = "deserialize_optional")]
    pub type_: Option<String>,
    /// The name of the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub name: Option<String>,
    /// The mod ID.
    #[serde(deserialize_with = "deserialize_optional")]
    pub modid: Option<String>,
    /// The version of the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub version: Option<String>,
    /// The network version of the mod.
    #[serde(rename = "networkVersion", deserialize_with = "deserialize_optional")]
    pub network_version: Option<String>,
    /// The texture size of the mod.
    #[serde(rename = "textureSize", deserialize_with = "deserialize_optional")]
    pub texture_size: Option<u32>,
    /// The description of the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub description: Option<String>,
    /// The website of the mod.
    #[serde(deserialize_with = "deserialize_optional", flatten)]
    pub website: Option<String>,
    /// The authors of the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub authors: Option<Vec<String>>,
    /// The contributors to the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub contributors: Option<Vec<String>>,
    /// The side of the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub side: Option<String>,
    /// Whether the mod is required on the client.
    #[serde(rename = "requiredOnClient", deserialize_with = "deserialize_optional")]
    pub required_on_client: Option<bool>,
    /// Whether the mod is required on the server.
    #[serde(rename = "requiredOnServer", deserialize_with = "deserialize_optional")]
    pub required_on_servers: Option<bool>,
    /// The dependencies of the mod.
    #[serde(deserialize_with = "deserialize_optional")]
    pub dependencies: Option<HashMap<String, String>>,
}

impl Default for ModInfo {
    /// Provides default values for `ModInfo`.
    fn default() -> Self {
        ModInfo {
            type_: None,
            name: None,
            modid: None,
            version: None,
            network_version: None,
            texture_size: None,
            description: None,
            website: None,
            authors: None,
            contributors: None,
            side: None,
            required_on_client: None,
            required_on_servers: None,
            dependencies: None,
        }
    }
}

// Hacky way to deserialize optional fields.
// This is needed because the modinfo.json file can have optional fields. and sometimes the fields have invalid types and Serde throws a hissy fit.
fn deserialize_optional<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer).ok().flatten())
}
