// src/api/moddatahandler.rs

use serde::Deserialize;
use utils::{LogLevel, Logger};

#[derive(Deserialize, Debug)]
pub struct Release {
    pub releaseid: u32,
    pub mainfile: String,
    pub filename: String,
    pub fileid: u32,
    pub downloads: u32,
    pub tags: Vec<String>,
    pub modidstr: String,
    pub modversion: String,
    pub created: String,
}

#[derive(Deserialize, Debug)]
pub struct Screenshot {
    pub fileid: u32,
    pub mainfile: String,
    pub filename: String,
    pub thumbnailfilename: Option<String>,
    pub created: String,
}

#[derive(Deserialize, Debug)]
pub struct Mod {
    pub modid: u32,
    pub assetid: u32,
    pub name: String,
    pub text: String,
    pub author: String,
    pub urlalias: Option<String>,
    pub logofilename: Option<String>,
    pub logofile: Option<String>,
    pub homepageurl: Option<String>,
    pub sourcecodeurl: Option<String>,
    pub trailervideourl: Option<String>,
    pub issuetrackerurl: Option<String>,
    pub wikiurl: Option<String>,
    pub downloads: u32,
    pub follows: u32,
    pub trendingpoints: u32,
    pub comments: u32,
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub created: String,
    pub lastmodified: String,
    pub tags: Vec<String>,
    pub releases: Vec<Release>,
    pub screenshots: Vec<Screenshot>,
}

#[derive(Deserialize, Debug)]
pub struct ModData {
    pub statuscode: String,
    #[serde(rename = "mod")]
    pub mod_data: Mod,
}

pub struct ModDataHandler {
    logger: Logger,
}

impl ModDataHandler {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("ModHandler".to_string(), LogLevel::Info),
        }
    }

    pub fn parse_mod_data(&self, json_data: &str) -> Result<ModData, serde_json::Error> {
        let mod_data: ModData = serde_json::from_str(json_data)?;
        //self.logger
        //.log_default(&format!("Parsed mod data: {:?}", mod_data));
        Ok(mod_data)
    }

    pub fn get_mainfile_path<'a>(&self, mod_data: &'a ModData) -> Option<&'a str> {
        mod_data
            .mod_data
            .releases
            .first()
            .map(|release| release.mainfile.as_str())
    }
}
