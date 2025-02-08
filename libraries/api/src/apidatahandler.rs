use serde::Deserialize;
use serde::Serialize;
use utils::{LogLevel, Logger};

/// Struct representing a release of a mod.
#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    /// The ID of the release.
    pub releaseid: u32,
    /// The main file of the release.
    pub mainfile: String,
    /// The filename of the release.
    pub filename: String,
    /// The file ID of the release.
    pub fileid: u32,
    /// The number of downloads of the release.
    pub downloads: u32,
    /// The tags associated with the release.
    pub tags: Vec<String>,
    /// The mod ID string.
    pub modidstr: String,
    /// The version of the mod.
    pub modversion: String,
    /// The creation date of the release.
    pub created: String,
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
    pub modid: u32,
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
pub struct APIData {
    /// The status code of the API response.
    pub statuscode: String,
    /// The mod data.
    #[serde(rename = "mod")]
    pub mod_data: Mod,
}

/// Struct to handle API data operations.
pub struct APIDataHandler {
    /// Logger instance for logging API data operations.
    logger: Logger,
}

impl APIDataHandler {
    /// Creates a new `APIDataHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `APIDataHandler` instance with a default logger.
    pub fn new() -> Self {
        Self {
            logger: Logger::new("ModHandler".to_string(), LogLevel::Info),
        }
    }

    /// Parses mod data from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json_data` - A string slice representing the JSON data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `APIData` or a `serde_json::Error`.
    pub fn parse_mod_data(&self, json_data: &str) -> Result<APIData, serde_json::Error> {
        let mod_data: APIData = serde_json::from_str(json_data)?;
        //self.logger
        //.log_default(&format!("Parsed mod data: {:?}", mod_data));
        Ok(mod_data)
    }

    /// Gets the main file path from the mod data.
    ///
    /// # Arguments
    ///
    /// * `mod_data` - A reference to the `APIData` instance.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the main file path if it exists, or `None` if it does not.
    pub fn get_mainfile_path<'a>(&self, mod_data: &'a APIData) -> Option<&'a str> {
        mod_data
            .mod_data
            .releases
            .first()
            .map(|release| release.mainfile.as_str())
    }
}
