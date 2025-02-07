use tokio;

use api::query::OrderBy;
use api::query::Query;
use api::{FileManager, VintageAPIHandler};
use utils::{LogLevel, Logger};

use std::path::Path;
use tokio::io::AsyncWriteExt;

use api::ModDataHandler;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestOrIOError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[tokio::main]
async fn main() -> Result<(), RequestOrIOError> {
    let api_client = VintageAPIHandler::new();
    let logger = Logger::new("Main".to_string(), LogLevel::Info);
    let encoder = utils::Encoder::new();
    let mod_handler = ModDataHandler::new();

    let mod_string = encoder.encode_mod_string(&[3213, 3214, 3217, 3215]);
    let decoded_mod_string = encoder.decode_mod_string(mod_string).unwrap();

    let data = api_client.get_mod_from_id(3209).await?;
    let data_from_name = api_client.get_mod_from_name("CarryCapacity").await?;

    let query = Query::new().with_order_by(OrderBy::last_released).build();

    let search_results = api_client.search_mods(query).await?;

    for modid in decoded_mod_string {
        let mod_data_json = api_client.get_mod_from_id(modid).await?;
        let mod_data = mod_handler.parse_mod_data(&mod_data_json)?;

        if let Some(mainfile_path) = mod_handler.get_mainfile_path(&mod_data) {
            logger.log_default(&format!("Main file path: {}", mainfile_path));
            // Extract the file name from the path
            let file_name = Path::new(mainfile_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("default.zip");
            logger.log_default(&format!("File name: {}", file_name));
            // Log the full path to be used
            let full_path = format!("mods/{}", file_name);
            logger.log_default(&format!("Full path: {}", full_path));
            // Download to directory called mods
            let file_stream = api_client
                .get_filestream(mainfile_path.parse().unwrap())
                .await?;
            FileManager::new()
                .save_file(&full_path, file_stream)
                .await?;
        }
    }

    //logger.log(LogLevel::Warn, &*data);
    //logger.log(LogLevel::Info, &*data_from_name);
    //logger.log(LogLevel::Info, &*search_results);

    Ok(())
}
