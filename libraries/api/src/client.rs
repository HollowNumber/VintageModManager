use reqwest::Client;
use utils::{LogLevel, Logger};

pub struct VintageAPIHandler {
    client: Client,
    // http://mods.vintagestory.at
    api_url: String,
    logger: Logger,
}

impl VintageAPIHandler {
    pub fn new() -> Self {
        let client = Client::new();
        let logger = Logger::new("ModAPIHandler".to_string(), LogLevel::Info);
        let url = "http://mods.vintagestory.at".to_string();
        Self {
            client,
            api_url: url,
            logger,
        }
    }

    pub async fn get_mod_from_id(&self, id: u16) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mod/{}", &self.api_url, id);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    // Todo: Implement get_mod_from_name
    pub async fn get_mod_from_name(&self, name: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mod/{}", &self.api_url, name);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    pub async fn get_mods(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mods", &self.api_url);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    pub async fn search_mods(&self, query: String) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/mods?{}", &self.api_url, query);
        self.logger.log(LogLevel::Info, &*url);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    pub async fn get_filestream(&self, file_path: String) -> Result<bytes::Bytes, reqwest::Error> {
        let url = format!("{}/{}", &self.api_url, file_path);
        let resp = self.client.get(&url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes)
    }
}
