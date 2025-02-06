use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct APIClient {
    pub client: Client,
    // http://mods.vintagestory.at/api
    pub api_url: String,
}

impl APIClient {
    pub fn new(url: String) -> Self {
        let client = Client::new();
        Self {
            client,
            api_url: url,
        }
    }

    pub async fn get_mod_from_id(&self, id: u16) -> Result<String, reqwest::Error> {
        let url = format!("{}/mod/{}", &self.api_url, id);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    // Todo: Implement get_mod_from_name
    pub async fn get_mod_from_name(&self, name: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/mod/{}", &self.api_url, name);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }

    pub async fn get_mods(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/mods", &self.api_url);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }


    pub async fn search_mods(&self, query: &str) -> Result<String, reqwest::Error> {
        let query_string = query.to_query_string();
        let url = format!("{}/search?{}", &self.api_url, query_string);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }
}
