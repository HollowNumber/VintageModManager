use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct ModInfo {
    #[serde(rename = "type", deserialize_with = "deserialize_optional")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub modid: Option<String>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub version: Option<String>,
    #[serde(rename = "networkVersion", deserialize_with = "deserialize_optional")]
    pub network_version: Option<String>,
    #[serde(rename = "textureSize", deserialize_with = "deserialize_optional")]
    pub texture_size: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub website: Option<String>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub authors: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub contributors: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub side: Option<String>,
    #[serde(rename = "requiredOnClient", deserialize_with = "deserialize_optional")]
    pub required_on_client: Option<bool>,
    #[serde(rename = "requiredOnServer", deserialize_with = "deserialize_optional")]
    pub required_on_servers: Option<bool>,
    #[serde(deserialize_with = "deserialize_optional")]
    pub dependencies: Option<HashMap<String, String>>,
}

impl Default for ModInfo {
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

fn deserialize_optional<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer).ok().flatten())
}
