use crate::api::releases::GameVersionsResponse;
use crate::api::{ModApiResponse, ModInfo};
use crate::api::{ModSearchResponse, Release};
use crate::config::VersionMapping;
use crate::utils::{LogLevel, Logger};
use reqwest::Client;
use std::fmt::Display;
use thiserror::Error;

const VINTAGE_STORY_URL: &str = "https://mods.vintagestory.at";

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Mod not found: {0}")]
    ModNotFound(String),
    #[error("API returned error status: {status}")]
    ApiError { status: u16 },
}

/// Struct to handle interactions with the Vintage Story API.
pub struct VintageApiHandler {
    /// HTTP client for making requests.
    client: Client,
    /// Base URL of the Vintage Story API.
    api_url: String,
    /// Logger instance for logging API interactions.
    logger: Logger,
}

impl VintageApiHandler {
    /// Creates a new `VintageAPIHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `VintageAPIHandler` instance with a default logger and API URL.
    pub fn new(verbose: bool) -> Self {
        let client = Client::new();
        let logger = Logger::new(
            "VintageAPIHandler".to_string(),
            LogLevel::Info,
            None,
            verbose,
        );
        Self {
            client,
            api_url: VINTAGE_STORY_URL.to_string(),
            logger,
        }
    }

    /// Fetches a mod by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the mod to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the mod data as a `String` or an error.
    pub async fn get_mod<T>(&self, identifier: T) -> Result<ModApiResponse, ClientError>
    where
        T: Display + ToString,
    {
        let url = format!("{}/api/mod/{}", &self.api_url, identifier);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;

        Self::parse_to_api_response(identifier, &body)
    }

    fn parse_to_api_response<T>(identifier: T, body: &str) -> Result<ModApiResponse, ClientError>
    where
        T: ToString,
    {
        match serde_json::from_str::<ModApiResponse>(body) {
            Ok(mod_res) => Ok(mod_res),
            Err(_) => {
                // If that fails, check if it's a 404 error response
                if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(body) {
                    if let Some(status_code) = error_response.get("statuscode") {
                        if status_code == 404 {
                            return Err(ClientError::ModNotFound(identifier.to_string()));
                        }
                    }
                }

                // If it's neither a valid response nor a 404, return parsing error
                Err(ClientError::Json(
                    serde_json::from_str::<ModApiResponse>(body).unwrap_err(),
                ))
            }
        }
    }

    /// Fetches all mods.
    ///
    /// # Returns
    ///
    /// A `Result` containing the mods data as a `String` or an error.
    pub async fn fetch_mods(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mods", &self.api_url);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    /// Searches for mods based on a query string.
    ///
    /// # Arguments
    ///
    /// * `query` - The query string to search for mods.
    ///
    /// # Returns
    ///
    /// A `Result` containing the search results as a `String` or an error.
    pub async fn search_mods(&self, query: String) -> Result<ModSearchResponse, ClientError> {
        let url = format!("{}/api/mods?{}", &self.api_url, query);
        self.logger.log(LogLevel::Info, &url);
        let resp = self.client.get(&url).send().await?;
        let search_results: ModSearchResponse = serde_json::from_str(&resp.text().await?).unwrap();
        Ok(search_results)
    }

    /// Fetches a file stream from a given file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file data as `Vector<u8>` or an error.
    pub async fn fetch_file_stream(&self, file_path: String) -> Result<Vec<u8>, ClientError> {
        let url = format!("{}/{}", &self.api_url, file_path);
        let resp = self.client.get(&url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_file_stream_from_url(&self, url: String) -> Result<Vec<u8>, ClientError> {
        let resp = self.client.get(&url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Compares local Modinfo with the API Modinfo for updates.
    ///
    /// # Arguments
    /// * `modinfo` - The Modinfo struct to compare.
    ///
    /// # Returns
    /// A tuple containing a boolean indicating if an update is available and a string with the version.
    pub async fn check_for_mod_update(
        &self, mod_info: &ModInfo,
    ) -> Result<(bool, Release), ClientError> {
        let mod_id = mod_info.modid.clone().expect("Mod id not found");
        self.logger
            .log_default(&format!("Checking for updates for mod: {mod_id}"));
        let api_mod = self.get_mod(&mod_id).await?;
        self.logger.log_default(&format!(
            "Mod info version: {:?} -- API version: {:?}",
            mod_info.version, api_mod.mod_data.releases[0].modversion
        ));

        let is_update_available = mod_info.version.clone().expect("Version not found")
            != api_mod.mod_data.releases[0].modversion.clone().unwrap();

        Ok((is_update_available, api_mod.mod_data.releases[0].clone()))
    }

    pub async fn fetch_game_versions(&self) -> Result<Vec<VersionMapping>, reqwest::Error> {
        self.logger.log_default("Fetching game versions");

        let url = format!("{}/api/gameversions", &self.api_url);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        let versions: GameVersionsResponse = serde_json::from_str(&body).unwrap();

        let mut version_mappings = Vec::new();

        for (index, version) in versions.gameversions.iter().enumerate() {
            version_mappings.push(VersionMapping::new(index as i64, version.name.clone()));
        }

        Ok(version_mappings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::query::Query;

    #[tokio::test]
    #[ignore]
    async fn test_get_mod_from_id() {
        let api = VintageApiHandler::new(false);
        let mod_data = api.get_mod(3351).await.unwrap();
        // assert!(mod_data.contains("Crude Arrows"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_mod_from_name() {
        let api = VintageApiHandler::new(false);
        let mod_data = api.get_mod("crudetoflintarrow").await.unwrap();
        todo!("Fix this");
        //assert!(mod_data.contains("Crude Arrows"));
    }

    #[tokio::test]
    async fn test_get_mods() {
        let api = VintageApiHandler::new(false);
        let mods = api.fetch_mods().await.unwrap();
        assert!(mods.contains("mods"));
    }

    #[tokio::test]
    async fn test_search_mods() {
        let api = VintageApiHandler::new(false);
        let query = Query::new().with_text(&vec!["jack".into()]).build();

        let search_results = api.search_mods(query).await.unwrap();
        assert_eq!(search_results.statuscode, "200");
        assert!(!search_results.mods.is_empty());
        assert!(
            search_results
                .mods
                .iter()
                .any(|m| m.name.to_lowercase().contains("jack"))
        );
    }

    #[tokio::test]
    async fn test_get_filestream() {
        let api = VintageApiHandler::new(false);
        let file = api
            .fetch_file_stream("api/mod/1".to_string())
            .await
            .unwrap();
        assert!(!file.is_empty());
    }
}
