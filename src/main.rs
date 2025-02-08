use tokio;

use api::query::OrderBy;
use api::query::Query;
use api::{ModInfo, VintageAPIHandler};
use utils::{get_config_dir, FileManager};
use utils::{LogLevel, Logger};

use tokio::io::AsyncWriteExt;

use api::APIDataHandler;
use thiserror::Error;

// Will not be used in the final version
#[derive(Error, Debug)]
pub enum RequestOrIOError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Bincode Error: {0}")]
    Bincode(#[from] Box<bincode::Error>),
    #[error("Var Error: {0}")]
    Var(#[from] std::env::VarError),
}

#[tokio::main]
async fn main() -> Result<(), RequestOrIOError> {
    let api_client = VintageAPIHandler::new();
    let logger = Logger::new("Main".to_string(), LogLevel::Info);
    let encoder = utils::Encoder::new();
    let mod_handler = APIDataHandler::new();

    let mod_string = encoder.encode_mod_string(&[3213, 3214, 3217, 3215]);
    let decoded_mod_string = encoder.decode_mod_string(mod_string).unwrap();

    let data = api_client.get_mod_from_id(3209).await?;
    let data_from_name = api_client.get_mod_from_name("CarryCapacity").await?;

    let query = Query::new().with_order_by(OrderBy::LastReleased).build();

    let search_results = api_client.search_mods(query).await?;
    /*
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
    }*/

    let filehandler = FileManager::new();
    let zip_path = "mods/polylocustsv1.0.0.zip".to_string();
    let zip_file = filehandler.read_modinfo_from_zip(&zip_path)?;
    let zip_file = String::from_utf8(zip_file).unwrap().to_lowercase();
    // Parse the modinfo.json file
    let modinfo: ModInfo = serde_json::from_str(&zip_file)?;
    logger.log_default(&format!("Zip file: {:?}", modinfo));

    logger.log(LogLevel::Info, &get_config_dir());

    //logger.log(LogLevel::Warn, &*data);
    //logger.log(LogLevel::Info, &*data_from_name);
    //logger.log(LogLevel::Info, &*search_results);

    Ok(())
}
