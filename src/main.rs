mod api;
mod utils;

use clap::Parser;
use rayon::prelude::*;
use tokio;

use api::{APIData, ModInfo, VintageAPIHandler};
use utils::{
    get_vintage_mods_dir, Commands, Encoder, EncoderData, FileManager, LogLevel, Logger, CLI,
};

use spinners::{Spinner, Spinners};

use tokio::io::AsyncWriteExt;

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
    let cli = CLI::parse();

    let verbose = cli.verbose.unwrap_or(false);

    let api = VintageAPIHandler::new(verbose);
    let file_manager = FileManager::new(verbose);
    let encoder = Encoder::new(verbose);
    let logger = Logger::new("Main".to_string(), LogLevel::Info, None, verbose);

    match cli.command {
        Some(Commands::Import {
            mod_string,
            multi_thread,
        }) => {
            // ensure the mod_string exists
            let mod_string = mod_string.expect("No mod string provided");
            logger.log_default(&format!("Importing mods from mod string: {}", mod_string));

            let mut spinner = Spinner::new(Spinners::Dots9, "Decoding mods...".into());
            let decoded = encoder
                .decode_mod_string(mod_string)
                .expect("Failed to decode mod string");
            spinner.stop();
            println!();

            println!("Downloading mods:");
            let progress_bar = indicatif::ProgressBar::new(decoded.len() as u64);

            for mod_data in decoded {
                let modinfo = api.get_mod_from_name(&mod_data.mod_id).await?;
                let modinfo: APIData = serde_json::from_str(&modinfo)?;
                let modinfo = modinfo.mod_data;

                // TODO: Currently i only download the newest release, this should be changed to download the same release denominated in the mod string
                let mod_path =
                    format!("{}{}", get_vintage_mods_dir(), modinfo.releases[0].filename);

                let mod_bytes = api
                    .get_filestream_from_url(modinfo.releases[0].mainfile.clone())
                    .await?;

                file_manager.save_file(&mod_path, mod_bytes).await?;
                progress_bar.inc(1);
            }

            progress_bar.finish();
        }

        Some(Commands::Export { export }) => {
            let mods = file_manager
                .get_modinfo_from_mods_folder()
                .await?
                .into_iter()
                .map(|mod_slice| {
                    let mod_string = std::str::from_utf8(&mod_slice).unwrap().to_lowercase();
                    let modinfo: ModInfo = serde_json::from_str(&mod_string).unwrap();
                    EncoderData {
                        mod_id: modinfo.modid.unwrap(),
                        mod_version: modinfo.version.unwrap(),
                    }
                })
                .collect::<Vec<EncoderData>>();

            let encoded = encoder.encode_mod_string(&mods);
            println!("{}", encoded);
        }

        Some(Commands::Update { check, update }) => {
            println!("Checking for updates");
        }

        _ => {}
    }

    Ok(())
}
