use serde::Serialize;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, de};
use std::fmt;

/// Struct representing a release of a mod.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Release {
    /// The ID of the release.
    pub releaseid: Option<u32>,
    /// The main file of the release.
    pub mainfile: Option<String>,
    /// The filename of the release.
    #[serde(deserialize_with = "deserialize_filename")]
    pub filename: Option<String>,
    /// The file ID of the release.
    pub fileid: Option<u32>,
    /// The number of downloads of the release.
    pub downloads: Option<u32>,
    /// The tags associated with the release.
    pub tags: Vec<String>,
    /// The mod ID string.
    pub modidstr: Option<String>,
    /// The version of the mod.
    pub modversion: Option<String>,
    /// The creation date of the release.
    pub created: Option<String>,
}

impl Default for Release {
    fn default() -> Self {
        Self {
            releaseid: Some(0),
            mainfile: Option::from("".to_string()),
            filename: None,
            fileid: Some(0),
            downloads: Some(0),
            tags: vec![],
            modidstr: Some("".to_string()),
            modversion: None,
            created: Some("".to_string()),
        }
    }
}

/// Struct representing a screenshot of a mod.
#[derive(Serialize, Deserialize, Debug)]
pub struct Screenshot {
    /// The file ID of the screenshot.
    pub fileid: u32,
    /// The main file of the screenshot.
    pub mainfile: String,
    /// The filename of the screenshot.
    pub filename: String,
    /// The optional thumbnail filename of the screenshot.
    pub thumbnailfilename: Option<String>,
    /// The creation date of the screenshot.
    pub created: String,
}

/// Struct representing a mod.
#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    /// The mod ID.
    pub modid: u16,
    /// The asset ID of the mod.
    pub assetid: u32,
    /// The name of the mod.
    pub name: String,
    /// The text description of the mod.
    pub text: String,
    /// The author of the mod.
    pub author: String,
    /// The optional URL alias of the mod.
    pub urlalias: Option<String>,
    /// The optional logo filename of the mod.
    pub logofilename: Option<String>,
    /// The optional logo file of the mod.
    pub logofile: Option<String>,
    /// The optional homepage URL of the mod.
    pub homepageurl: Option<String>,
    /// The optional source code URL of the mod.
    pub sourcecodeurl: Option<String>,
    /// The optional trailer video URL of the mod.
    pub trailervideourl: Option<String>,
    /// The optional issue tracker URL of the mod.
    pub issuetrackerurl: Option<String>,
    /// The optional wiki URL of the mod.
    pub wikiurl: Option<String>,
    /// The number of downloads of the mod.
    pub downloads: u32,
    /// The number of follows of the mod.
    pub follows: u32,
    /// The trending points of the mod.
    pub trendingpoints: u32,
    /// The number of comments on the mod.
    pub comments: u32,
    /// The side of the mod.
    pub side: String,
    /// The type of the mod.
    #[serde(rename = "type")]
    pub type_: String,
    /// The creation date of the mod.
    pub created: String,
    /// The last modified date of the mod.
    pub lastmodified: String,
    /// The tags associated with the mod.
    pub tags: Vec<String>,
    /// The releases of the mod.
    pub releases: Vec<Release>,
    /// The screenshots of the mod.
    pub screenshots: Vec<Screenshot>,
}

/// Struct representing the API data.
#[derive(Serialize, Deserialize, Debug)]
pub struct ModApiResponse {
    /// The status code of the API response.
    pub statuscode: String,
    /// The mod data.
    #[serde(rename = "mod")]
    pub mod_data: Mod,
}

fn deserialize_filename<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct FilenameVisitor;

    impl<'de> Visitor<'de> for FilenameVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or an integer")
        }

        fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(String::new()))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(String::new()))
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(FilenameVisitor)
        }
    }
    deserializer.deserialize_option(FilenameVisitor)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModSearchResult {
    pub modid: u16,
    pub assetid: u32,
    pub downloads: Option<u32>,
    pub follows: Option<u32>,
    pub trendingpoints: Option<u32>,
    pub comments: Option<u32>,
    pub name: String,
    pub summary: Option<String>,
    pub modidstrs: Vec<String>,
    pub author: String,
    pub urlalias: Option<String>,
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub logo: Option<String>,
    pub tags: Vec<String>,
    pub lastreleased: Option<String>,
}

/// Struct representing the search API response
#[derive(Serialize, Deserialize, Debug)]
pub struct ModSearchResponse {
    pub statuscode: String,
    pub mods: Vec<ModSearchResult>,
}

impl fmt::Display for ModSearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {} ({} downloads)",
            self.name,
            self.author,
            self.downloads.unwrap_or(0)
        )
    }
}
