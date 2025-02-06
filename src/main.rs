use tokio;

mod utils;

use crate::utils::{APIClient, LogLevel, Logger};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let base_url = "http://mods.vintagestory.at/api";
    let api_client = APIClient::new(base_url.parse().unwrap());
    let logger = Logger::new("Main".to_string(), LogLevel::Info);

    let data = api_client.get_mod_from_id(3203).await?;

    logger.log(LogLevel::Warn, &*data);

    Ok(())
}
