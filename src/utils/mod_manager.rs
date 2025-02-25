use crate::api::{
    ModApiResponse, ModInfo, ModSearchResult, OrderBy, Query, Release, VintageApiHandler,
};
use crate::utils::cli::IsAllNone;
use crate::utils::encoding::EncodingError;
use crate::utils::files::FileError;
use crate::utils::terminal::Terminal;
use crate::utils::{
    Cli, CliFlags, Commands, DownloadFlags, Encoder, EncoderData, FileManager, LogLevel, Logger,
    ProgressBarWrapper, get_vintage_mods_dir,
};
use clap::Parser;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModManagerError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Var Error: {0}")]
    Var(#[from] std::env::VarError),
    #[error("No Releases Error")]
    NoReleases,
    #[error("Invalid mod path: {0}")]
    InvalidModPath(String),
    #[error("Missing modinfo")]
    MissingModInfo,
    #[error("File Error: {0}")]
    File(#[from] FileError),
    #[error("Encoding Error: {0}")]
    Encoding(#[from] EncodingError),
    #[error("Dialog Error: {0}")]
    Dialog(#[from] dialoguer::Error),
}

pub struct ModManager {
    api: VintageApiHandler,
    file_manager: FileManager,
    encoder: Encoder,
    logger: Logger,
}

enum SelectionResult {
    Continue,
    Break,
    NoResults,
}

impl ModManager {
    pub fn new(verbose: bool) -> Self {
        Self {
            api: VintageApiHandler::new(verbose),
            file_manager: FileManager::new(verbose),
            encoder: Encoder::new(verbose),
            logger: Logger::new("ModManager".to_string(), LogLevel::Info, None, verbose),
        }
    }

    pub async fn run() -> Result<(), ModManagerError> {
        let cli = Cli::parse();
        let verbose = cli.verbose.unwrap_or(false);
        let mod_manager = ModManager::new(verbose);

        match cli.command {
            Some(Commands::Download {
                mod_string,
                mods,
                mod_,
            }) => {
                mod_manager
                    .import_mods(Some(DownloadFlags {
                        mod_string,
                        mods,
                        mod_,
                    }))
                    .await?;
            }

            Some(Commands::Export {
                exclude,
                include,
                mod_,
                interactive,
            }) => {
                let options = CliFlags {
                    exclude,
                    include,
                    mod_,
                };

                mod_manager.handle_export(interactive, options).await?;
            }

            Some(Commands::Update {
                exclude,
                include,
                mod_,
            }) => {
                mod_manager
                    .update_mods(CliFlags {
                        exclude,
                        include,
                        mod_,
                    })
                    .await?;
            }

            _ => {}
        }

        Ok(())
    }

    async fn import_mods(&self, options: Option<DownloadFlags>) -> Result<(), ModManagerError> {
        let options = options.ok_or(ModManagerError::MissingModInfo)?;

        if let Some(mod_string) = &options.mod_string {
            self.download_mod_string(mod_string).await?;
        }

        if let Some(mods) = &options.mods {
            self.download_mods(mods).await?;
        }

        if let Some(mod_) = &options.mod_ {
            self.download_mod(mod_).await?;
        }

        if options.is_all_none() {
            self.show_paginated_mods().await?;
        }

        Ok(())
    }

    async fn handle_export(
        &self, interactive: Option<bool>, option: CliFlags,
    ) -> Result<(), ModManagerError> {
        let mods: Vec<(ModInfo, PathBuf)> = self.file_manager.collect_mods(&Some(option)).await?;

        let selected_mods = if interactive.unwrap_or(false) {
            let mod_names: Vec<_> = mods
                .iter()
                .map(|(info, _)| info.name.as_deref().unwrap_or("Unknown"))
                .collect();

            let selections = Terminal::multi_select("Select mods to export", &mod_names);
            selections
                .into_iter()
                .map(|idx| mods[idx].clone())
                .collect()
        } else {
            mods
        };

        let encoder_data = self.create_encoder_data(&selected_mods)?;
        let encoded = self.encoder.encode_mod_string(&encoder_data);

        self.logger
            .log_default(&format!("Exported {} mods", selected_mods.len()));
        println!("{}", encoded);
        Ok(())
    }

    fn create_encoder_data(
        &self, mods: &[(ModInfo, PathBuf)],
    ) -> Result<Vec<EncoderData>, ModManagerError> {
        mods.iter()
            .map(|(mod_info, _)| {
                self.logger
                    .log_default(&format!("Creating encoder data for: {:?}", mod_info));
                let mod_id = mod_info
                    .modid
                    .as_ref()
                    .ok_or_else(|| ModManagerError::InvalidModPath("Missing mod ID".to_string()))?;
                let version = mod_info.version.as_ref().ok_or_else(|| {
                    ModManagerError::InvalidModPath("Missing mod version".to_string())
                })?;

                Ok(EncoderData {
                    mod_id: mod_id.clone(),
                    mod_version: version.clone(),
                })
            })
            .collect()
    }

    async fn update_mods(&self, mod_options: CliFlags) -> Result<(), ModManagerError> {
        let mods = self.file_manager.collect_mods(&Some(mod_options)).await?;
        let vintage_mods_dir = get_vintage_mods_dir()?;

        println!("Checking for updates...");
        for (mod_info, path) in mods {
            self.process_mod_update(&mod_info, path, &vintage_mods_dir)
                .await;
        }

        Ok(())
    }

    async fn process_mod_update(&self, mod_info: &ModInfo, path: PathBuf, mods_dir: &Path) {
        let name = mod_info.name.as_deref().unwrap_or("Unknown");
        let version = mod_info.version.as_deref().unwrap_or("Unknown");

        match self.check_and_get_update(mod_info, name, version).await {
            Some(release) => {
                self.handle_mod_update(name, version, path, mods_dir, release)
                    .await
            }
            None => println!(
                "No update available for mod: {} - Current version: {}",
                name, version
            ),
        }
    }

    async fn check_and_get_update(
        &self, mod_info: &ModInfo, name: &str, version: &str,
    ) -> Option<Release> {
        match self.api.check_for_mod_update(mod_info).await {
            Ok((true, release)) => {
                let new_version = release.modversion.as_deref().unwrap_or("Unknown");
                println!(
                    "Update available for mod: {} - Current version: {} - New version: {}",
                    name, version, new_version
                );
                Some(release)
            }
            Ok((false, _)) => None,
            Err(e) => {
                eprintln!("Failed to check updates for {}: {}", name, e);
                None
            }
        }
    }

    async fn handle_mod_update(
        &self, name: &str, _version: &str, path: PathBuf, mods_dir: &Path, release: Release,
    ) {
        // Delete old mod
        if let Err(e) = self.delete_old_mod(&path).await {
            eprintln!("Failed to delete old mod: {}", e);
            return;
        }

        // Get new mod path
        let new_mod_path = match self.get_new_mod_path(mods_dir, &release, name) {
            Some(path) => path,
            None => return,
        };

        // Download and save new mod
        self.download_and_save_mod(name, &new_mod_path, &release)
            .await;
    }

    async fn show_paginated_mods(&self) -> Result<(), ModManagerError> {
        let page_size = 50;
        let mut current_filter = String::new();
        let mods = self.fetch_initial_mods().await?;

        while !mods.is_empty() {
            match self
                .handle_mod_selection(&mods, &mut current_filter, page_size)
                .await?
            {
                SelectionResult::Continue => continue,
                SelectionResult::Break => break,
                SelectionResult::NoResults => return Ok(()),
            }
        }

        Ok(())
    }

    async fn fetch_initial_mods(&self) -> Result<Vec<ModSearchResult>, ModManagerError> {
        let query = Query::new().with_order_by(OrderBy::Downloads).build();
        let search_results = self.api.search_mods(query).await?;
        Ok(search_results.mods)
    }

    fn filter_mods<'a>(
        &self, mods: &'a [ModSearchResult], filter: &str, page_size: usize,
    ) -> Vec<&'a ModSearchResult> {
        mods.iter()
            .filter(|m| {
                filter.is_empty()
                    || m.name.to_lowercase().contains(&filter.to_lowercase())
                    || m.author.to_lowercase().contains(&filter.to_lowercase())
            })
            .take(page_size)
            .collect()
    }

    fn create_display_options(&self, mods: &[&ModSearchResult]) -> Vec<String> {
        let mut options: Vec<String> = mods
            .iter()
            .map(|m| {
                format!(
                    "{} by {} ({} downloads)",
                    m.name,
                    m.author,
                    m.downloads.unwrap_or(0)
                )
            })
            .collect();

        options.push("--- Filter mods ---".into());
        options.push("--- Exit ---".into());
        options
    }

    async fn handle_mod_selection(
        &self, mods: &[ModSearchResult], current_filter: &mut String, page_size: usize,
    ) -> Result<SelectionResult, ModManagerError> {
        let displayed_mods = self.filter_mods(mods, current_filter, page_size);

        if displayed_mods.is_empty() {
            println!("No mods found matching filter: {}", current_filter);
            return Ok(SelectionResult::NoResults);
        }

        let options = self.create_display_options(&displayed_mods);

        match Terminal::select("Select a mod (use / to search, ESC to exit)", &options) {
            Some(selection) if selection >= displayed_mods.len() => {
                match selection - displayed_mods.len() {
                    0 => {
                        self.handle_navigation_selection(0, current_filter)?;
                        Ok(SelectionResult::Continue)
                    }
                    1 => Ok(SelectionResult::Break), // Exit option
                    _ => Ok(SelectionResult::Continue),
                }
            }
            Some(selection) => {
                self.handle_mod_download(displayed_mods[selection]).await?;
                Ok(SelectionResult::Continue)
            }
            None => Ok(SelectionResult::Break),
        }
    }

    fn handle_navigation_selection(
        &self, nav_index: usize, current_filter: &mut String,
    ) -> Result<(), ModManagerError> {
        match nav_index {
            0 => {
                self.clear_screen()?;
                print!("Filter for mod: ");
                std::io::Write::flush(&mut std::io::stdout())?;
                *current_filter = Terminal::input("");
                Ok(())
            }
            1 => {
                // Exit option - this will be handled by the caller
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn handle_mod_download(
        &self, selected_mod: &ModSearchResult,
    ) -> Result<(), ModManagerError> {
        let mod_info = self.fetch_mod_info(&selected_mod.modidstrs[0]).await?;

        if Terminal::confirm(format!("Download mod: {}?", selected_mod.name)) {
            self.save_mod_file(&mod_info).await?;
            println!("Downloaded {}", selected_mod.name);
        }

        Ok(())
    }

    fn clear_screen(&self) -> Result<(), ModManagerError> {
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout())?;
        Ok(())
    }

    async fn delete_old_mod(&self, path: &PathBuf) -> Result<(), FileError> {
        println!("Deleting old mod: {}", path.display());
        self.file_manager.delete_file(path).await
    }

    fn get_new_mod_path(&self, mods_dir: &Path, release: &Release, name: &str) -> Option<PathBuf> {
        match &release.filename {
            Some(filename) => Some(mods_dir.join(filename)),
            None => {
                eprintln!("Missing filename for mod: {}", name);
                None
            }
        }
    }

    async fn download_and_save_mod(&self, name: &str, new_mod_path: &PathBuf, release: &Release) {
        let mod_bytes = match &release.mainfile {
            Some(url) => match self.api.fetch_file_stream_from_url(url.clone()).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Failed to download mod {}: {}", name, e);
                    return;
                }
            },
            None => {
                eprintln!("Missing download URL for mod: {}", name);
                return;
            }
        };

        if let Err(e) = self.file_manager.save_file(new_mod_path, &mod_bytes).await {
            eprintln!("Failed to save new mod {}: {}", name, e);
        }
    }

    async fn download_mod(&self, mod_data: &str) -> Result<(), ModManagerError> {
        let query = Query::new()
            .with_text(&vec![mod_data.to_string()])
            .with_order_by(OrderBy::Downloads)
            .build();

        let query_results = self.api.search_mods(query).await?;
        self.logger
            .log_default(&format!("Found {} mods", query_results.mods.len()));

        if let Some(selection) = Terminal::select("Select a mod to download", &query_results.mods) {
            let selected_mod = &query_results.mods[selection];
            let mod_info = self.fetch_mod_info(&selected_mod.modidstrs[0]).await?;

            if Terminal::confirm(format!("Download mod: {}?", selected_mod.name)) {
                self.save_mod_file(&mod_info).await?;
            }
        }

        Ok(())
    }

    async fn download_mods(&self, mods: &Vec<String>) -> Result<(), ModManagerError> {
        let query = Query::new()
            .with_text(mods)
            .with_order_by(OrderBy::Downloads)
            .build();
        self.logger
            .log_default(&format!("Searching for mods: {:?}", mods));

        let query_results = self.api.search_mods(query).await?;
        self.logger
            .log_default(&format!("Found {} mods", query_results.mods.len()));

        if query_results.mods.is_empty() {
            println!("No mods found, try again with different search terms");
            return Ok(());
        }

        let selections = Terminal::multi_select("Select mods to download", &query_results.mods);
        if !selections.is_empty() {
            let progress_bar = ProgressBarWrapper::new(selections.len() as u64);

            for selection in selections {
                let selected_mod = &query_results.mods[selection];
                let mod_info = self.fetch_mod_info(&selected_mod.modidstrs[0]).await?;
                self.save_mod_file(&mod_info).await?;
                progress_bar.println(format!("Downloaded mod: {}", selected_mod.name));
                progress_bar.inc(1);
            }

            progress_bar.finish_with_message("Finished downloading mods");
        }

        Ok(())
    }

    async fn download_mod_string(&self, mod_string: &str) -> Result<(), ModManagerError> {
        let decoded: Vec<EncoderData> = self.encoder.decode_mod_string(mod_string.to_owned())?;
        let progress_bar = ProgressBarWrapper::new(decoded.len() as u64);

        for mod_data in decoded {
            let mod_info = self.fetch_mod_info(&mod_data.mod_id).await?;
            progress_bar.set_message(format!("Downloading mod: {}", mod_info.mod_data.name));
            self.save_mod_file(&mod_info).await?;
            progress_bar.inc(1);
        }

        progress_bar.finish_with_message("Finished downloading mods");
        Ok(())
    }

    async fn fetch_mod_info(&self, mod_id: &String) -> Result<ModApiResponse, ModManagerError> {
        self.logger
            .log_default(&format!("Fetching mod info: {}", mod_id));
        let mod_info = self.api.get_mod_from_name(mod_id).await?;
        let mod_info: ModApiResponse = serde_json::from_str(&mod_info)?;
        Ok(mod_info)
    }

    async fn save_mod_file(&self, mod_info: &ModApiResponse) -> Result<(), ModManagerError> {
        let vintage_mods_dir = get_vintage_mods_dir()?;
        let release = &mod_info.mod_data.releases[0];
        let mod_path = vintage_mods_dir.join(release.filename.clone().unwrap());
        let mod_bytes = self
            .api
            .fetch_file_stream_from_url(release.mainfile.clone().unwrap())
            .await?;

        self.file_manager.save_file(&mod_path, &mod_bytes).await?;
        Ok(())
    }
}
