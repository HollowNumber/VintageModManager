use crate::{APIData, ModInfo};
use reqwest::Client;
use utils::{LogLevel, Logger};

/// Struct to handle interactions with the Vintage Story API.
pub struct VintageAPIHandler {
    /// HTTP client for making requests.
    client: Client,
    /// Base URL of the Vintage Story API.
    api_url: String,
    /// Logger instance for logging API interactions.
    logger: Logger,
}

impl VintageAPIHandler {
    /// Creates a new `VintageAPIHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `VintageAPIHandler` instance with a default logger and API URL.
    pub fn new() -> Self {
        let client = Client::new();
        let logger = Logger::new(
            "VintageAPIHandler".to_string(),
            LogLevel::Info,
            "logs/APIHandler.log",
        );
        let url = "http://mods.vintagestory.at".to_string();
        Self {
            client,
            api_url: url,
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
    /// # Todo
    ///
    /// Implement the `get_mod_from_name` method.
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
    pub async fn get_mods(&self) -> Result<String, reqwest::Error> {
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
    pub async fn search_mods(&self, query: String) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mods?{}", &self.api_url, query);
        self.logger.log(LogLevel::Info, &*url);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    /// Fetches a file stream from a given file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file data as `bytes::Bytes` or an error.
    pub async fn get_filestream(&self, file_path: String) -> Result<bytes::Bytes, reqwest::Error> {
        let url = format!("{}/{}", &self.api_url, file_path);
        let resp = self.client.get(&url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes)
    }

    /// Compares local Modinfo with the API Modinfo for updates.
    ///
    /// # Arguments
    /// * `modinfo` - The Modinfo struct to compare.
    ///
    /// # Returns
    /// A `bool` if the mod is up to date or not.
    pub async fn check_for_update(&self, modinfo: ModInfo) -> Result<bool, reqwest::Error> {
        let mod_id = modinfo.modid.expect("Modid not found");
        self.logger
            .log_default(&format!("Checking for updates for mod: {}", mod_id));
        let api_mod = self.get_mod_from_name(&mod_id).await?;
        let api_modinfo: APIData = serde_json::from_str(&api_mod).unwrap();
        self.logger.log_default(&format!(
            "Modinfo version: {:?} -- API version: {:?}",
            modinfo.version, api_modinfo.mod_data.releases[0].modversion
        ));

        if modinfo.version.expect("Version not found")
            == api_modinfo.mod_data.releases[0].modversion
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
