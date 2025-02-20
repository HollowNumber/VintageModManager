mod api;
mod utils;

use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
use tokio;

use api::{ModApiResponse, ModInfo, VintageAPIHandler};
use utils::{
    get_vintage_mods_dir, Commands, DownloadOptions, Encoder, EncoderData, FileManager, LogLevel,
    Logger, CLI,
};

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
            import_mods(
                Some(DownloadOptions {
                    mod_string,
                    mods,
                    mod_,
                    multi_thread,
                }),
                &api,
                &file_manager,
                &encoder,
                &logger,
            )
            .await?;
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
                latest_release.modversion.unwrap()
            );

            // Delete old mod
            println!("Deleting old mod: {}", path.display());
            file_manager.delete_file(&path).await?;

            let new_mod_path = format!(
                "{}{}",
                vintage_mods_dir,
                latest_release.filename.clone().unwrap()
            );

            let mod_bytes = api
                .fetch_file_stream_from_url(latest_release.mainfile.clone().unwrap())
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
    options: Option<DownloadOptions>,
    api: &VintageAPIHandler,
    file_manager: &FileManager,
    encoder: &Encoder,
    logger: &Logger,
) -> Result<(), RequestOrIOError> {
    let options = options.unwrap();

    if let Some(mod_string) = &options.mod_string {
        download_mod_string(mod_string, api, file_manager, encoder, logger).await?;
    }

    if let Some(mods) = &options.mods {
        download_mods(mods, api, file_manager, logger).await?;
    }

    if let Some(mod_) = &options.mod_ {
        download_mod(mod_, api, file_manager, logger).await?;
    }

    Ok(())
}

async fn download_mod(
    mod_data: &String,
    api: &VintageAPIHandler,
    file_manager: &FileManager,
    logger: &Logger,
) -> Result<(), RequestOrIOError> {
    let modinfo = fetch_mod_info(mod_data, api, logger).await?;
    save_mod_file(&modinfo, api, file_manager).await?;
    Ok(())
}

async fn download_mods(
    mods: &Vec<String>,
    api: &VintageAPIHandler,
    file_manager: &FileManager,
    logger: &Logger,
) -> Result<(), RequestOrIOError> {
    let progress_bar = indicatif::ProgressBar::new(mods.len() as u64);

    for mod_id in mods {
        let modinfo = fetch_mod_info(mod_id, api, logger).await?;
        save_mod_file(&modinfo, api, file_manager).await?;
        progress_bar.inc(1);
    }

    progress_bar.finish();
    Ok(())
}

async fn download_mod_string(
    mod_string: &String,
    api: &VintageAPIHandler,
    file_manager: &FileManager,
    encoder: &Encoder,
    logger: &Logger,
) -> Result<(), RequestOrIOError> {
    let decoded = encoder.decode_mod_string(mod_string.clone()).unwrap();
    let progress_bar = indicatif::ProgressBar::new(decoded.len() as u64);

    for mod_data in decoded {
        let modinfo = fetch_mod_info(&mod_data.mod_id, api, logger).await?;
        save_mod_file(&modinfo, api, file_manager).await?;
        progress_bar.inc(1);
    }

    progress_bar.finish();
    Ok(())
}

async fn fetch_mod_info(
    mod_id: &String,
    api: &VintageAPIHandler,
    logger: &Logger,
) -> Result<ModApiResponse, RequestOrIOError> {
    logger.log_default(&format!("Fetching mod info: {}", mod_id));
    let modinfo = api.get_mod_from_name(mod_id).await?;
    let modinfo: ModApiResponse = serde_json::from_str(&modinfo)?;
    Ok(modinfo)
}

async fn save_mod_file(
    modinfo: &ModApiResponse,
    api: &VintageAPIHandler,
    file_manager: &FileManager,
) -> Result<(), RequestOrIOError> {
    let vintage_mods_dir = get_vintage_mods_dir();
    let release = &modinfo.mod_data.releases[0];
    let mod_path = format!("{}{}", vintage_mods_dir, release.filename.clone().unwrap());

    println!("Downloading mod: {}", release.filename.clone().unwrap());

    let mod_bytes = api
        .fetch_file_stream_from_url(release.mainfile.clone().unwrap())
        .await?;

    file_manager.save_file(&mod_path, mod_bytes).await?;
    Ok(())
}
