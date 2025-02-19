mod api;
mod utils;

use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
use tokio;

use api::{ModApiResponse, ModInfo, VintageAPIHandler};
use utils::{
    get_vintage_mods_dir, Commands, Encoder, EncoderData, FileManager, LogLevel, Logger, CLI,
};

use spinners::{Spinner, Spinners};

use tokio::io::AsyncWriteExt;

use crate::utils::CliOptions;
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
        Some(Commands::Download {
            mod_string,
            multi_thread,
            mods,
            mod_,
        }) => {
            let mod_string = mod_string.expect("No mod string provided");
            import_mods(mod_string, &api, &file_manager, &encoder, &logger).await?;
        }

        Some(Commands::Export {
            all,
            exclude,
            include,
            mod_,
        }) => {
            let mods = file_manager
                .collect_mods(&Some(CliOptions {
                    all,
                    exclude,
                    include,
                    mod_,
                }))
                .await?;

            let encoder_data: Vec<EncoderData> = mods
                .iter()
                .map(|(modinfo, _)| EncoderData {
                    mod_id: modinfo.modid.clone().unwrap(),
                    mod_version: modinfo.version.clone().unwrap(),
                })
                .collect();
            let encoded = encoder.encode_mod_string(&encoder_data);
            println!("{}", encoded);
        }

        Some(Commands::Update {
            all,
            exclude,
            include,
            mod_,
        }) => {
            update_mods(
                &api,
                &file_manager,
                &encoder,
                &logger,
                Some(CliOptions {
                    all,
                    exclude,
                    include,
                    mod_,
                }),
            )
            .await?;
        }

        _ => {}
    }

    Ok(())
}

async fn update_mods(
    api: &VintageAPIHandler,
    file_manager: &FileManager,
    encoder: &Encoder,
    logger: &Logger,
    mod_options: Option<CliOptions>,
) -> Result<(), RequestOrIOError> {
    let mods: Vec<(ModInfo, PathBuf)> = file_manager.collect_mods(&mod_options).await?;
    let vintage_mods_dir = get_vintage_mods_dir();
    let mod_folder = file_manager
        .get_files_in_directory(&vintage_mods_dir)
        .await?;

    println!("Checking for updates...");
    for (_mod, path) in mods {
        let (update_available, latest_release) = api.check_for_mod_update(&_mod).await?;

        if update_available {
            println!(
                "Update available for mod: {} - Current version: {} - New version: {}",
                _mod.name.clone().unwrap(),
                _mod.version.clone().unwrap(),
                latest_release.modversion
            );

            // Delete old mod
            println!("Deleting old mod: {}", path.display());
            file_manager.delete_file(&path).await?;

            let new_mod_path = format!("{}{}", vintage_mods_dir, latest_release.filename);

            let mod_bytes = api
                .fetch_file_stream_from_url(latest_release.mainfile.clone())
                .await?;

            file_manager.save_file(&new_mod_path, mod_bytes).await?;
        } else {
            println!(
                "No update available for mod: {} - Current version: {}",
                _mod.name.clone().unwrap(),
                _mod.version.clone().unwrap()
            );
        }
    }

    Ok(())
}

async fn import_mods(
    mod_string: String,
    api: &VintageAPIHandler,
    file_manager: &FileManager,
    encoder: &Encoder,
    logger: &Logger,
) -> Result<(), RequestOrIOError> {
    logger.log_default(&format!("Importing mods from mod string: {}", mod_string));
    let vintage_mods_dir = get_vintage_mods_dir();

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
        let modinfo: ModApiResponse = serde_json::from_str(&modinfo)?;
        let modinfo = modinfo.mod_data;

        let release = modinfo
            .releases
            .iter()
            .find(|release| release.modversion == mod_data.mod_version)
            .expect("Mod release not found");

        let mod_path = format!("{}{}", vintage_mods_dir, release.filename);

        let mod_bytes = api
            .fetch_file_stream_from_url(release.mainfile.clone())
            .await?;

        file_manager.save_file(&mod_path, mod_bytes).await?;
        progress_bar.inc(1);
    }

    progress_bar.finish();
    Ok(())
}
