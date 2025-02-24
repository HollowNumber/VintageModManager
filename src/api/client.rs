use crate::api::{ModApiResponse, ModInfo};
use crate::api::{ModSearchResponse, Release};
use crate::utils::{LogLevel, Logger};
use reqwest::Client;

const VINTAGE_STORY_URL: &str = "http://mods.vintagestory.at";

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
    pub async fn get_mod_from_id(&self, id: u16) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mod/{}", &self.api_url, id);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    /// Fetches a mod by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the mod to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the mod data as a `String` or an error.
    ///
    pub async fn get_mod_from_name(&self, name: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mod/{}", &self.api_url, name);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
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
    pub async fn search_mods(&self, query: String) -> Result<ModSearchResponse, reqwest::Error> {
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
    pub async fn fetch_file_stream(&self, file_path: String) -> Result<Vec<u8>, reqwest::Error> {
        let url = format!("{}/{}", &self.api_url, file_path);
        let resp = self.client.get(&url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_file_stream_from_url(&self, url: String) -> Result<Vec<u8>, reqwest::Error> {
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
    ) -> Result<(bool, Release), reqwest::Error> {
        let mod_id = mod_info.modid.clone().expect("Mod id not found");
        self.logger
            .log_default(&format!("Checking for updates for mod: {}", mod_id));
        let api_mod = self.get_mod_from_name(&mod_id).await?;
        let api_mod_info: ModApiResponse = serde_json::from_str(&api_mod).unwrap();
        self.logger.log_default(&format!(
            "Mod info version: {:?} -- API version: {:?}",
            mod_info.version, api_mod_info.mod_data.releases[0].modversion
        ));

        let is_update_available = mod_info.version.clone().expect("Version not found")
            != api_mod_info.mod_data.releases[0]
                .modversion
                .clone()
                .unwrap();

        Ok((
            is_update_available,
            api_mod_info.mod_data.releases[0].clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::query::Query;

    #[tokio::test]
    async fn test_get_mod_from_id() {
        let api = VintageApiHandler::new(false);
        let mod_data = api.get_mod_from_id(3351).await.unwrap();
        assert!(mod_data.contains("Crude Arrows"));
    }

    #[tokio::test]
    async fn test_get_mod_from_name() {
        let api = VintageApiHandler::new(false);
        let mod_data = api.get_mod_from_name("crudetoflintarrow").await.unwrap();
        assert!(mod_data.contains("Crude Arrows"));
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
