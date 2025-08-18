use crate::api::ModInfo;
use crate::utils::{CliFlags, LogLevel, Logger, get_vintage_mods_dir};
use std::fs::File;

use std::io::Read;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReadDirStream;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("File not found: {0}")]
    FileNotFound(String),
}

/// Struct to manage file operations with logging.
pub struct FileManager {
    /// Logger instance for logging file operations.
    logger: Logger,
    base_path: PathBuf,
}

impl FileManager {
    /// Creates a new `FileManager` instance.
    ///
    /// # Returns
    ///
    /// A new `FileManager` instance with a default logger.
    pub fn new(verbose: bool) -> Self {
        Self {
            logger: Logger::new("FileManager".to_string(), LogLevel::Info, None, verbose),
            base_path: get_vintage_mods_dir().unwrap_or_default(),
        }
    }

    /// Saves a file asynchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to save.
    /// * `bytes` - The content to write to the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn save_file(&self, file_name: &PathBuf, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Saving file: {}", file_name.display()));
        let mut file = fs::File::create(file_name).await?;
        file.write_all(bytes).await?;
        Ok(())
    }

    async fn validate_path(&self, path: &PathBuf) -> Result<(), FileError> {
        if !path.starts_with(&self.base_path) {
            return Err(FileError::InvalidPath(path.to_owned()));
        }
        Ok(())
    }

    fn is_valid_mod_file(&self, path: &Path) -> bool {
        path.extension().is_some_and(|ext| ext == "zip")
    }

    /// Saves a file synchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to save.
    /// * `bytes` - The content to write to the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn save_file_sync(&self, file_name: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Saving file: {file_name}"));
        let mut file = File::create(file_name)?;
        std::io::Write::write_all(&mut file, bytes)?;
        Ok(())
    }

    /// Reads a file asynchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file content as `Bytes` or an error.
    pub async fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>, FileError> {
        if !path.exists() {
            return Err(FileError::FileNotFound(path.display().to_string()));
        }

        if !path.is_file() {
            return Err(FileError::InvalidPath(
                path.display().to_string().parse().unwrap(),
            ));
        }

        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }

    /// Reads a file synchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file content as `Bytes` or an error.
    pub fn read_file_sync(&self, file_name: &str) -> Result<Vec<u8>, std::io::Error> {
        self.logger
            .log_default(&format!("Reading file: {file_name}"));
        let mut file = File::open(file_name)?;
        let mut contents = Vec::new();
        Read::read_to_end(&mut file, &mut contents)?;
        Ok(contents)
    }

    /// Reads the `mod_info.json` file from a zip archive.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the zip file to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of `mod_info.json` as a `Vec<u8>` or an error.
    pub fn read_mod_info_from_zip(&self, path: &PathBuf) -> Result<Vec<u8>, FileError> {
        self.logger
            .log_default(&format!("Reading zip file: {}", path.display()));
        if !self.is_valid_mod_file(path) {
            return Err(FileError::InvalidPath(path.to_owned()));
        }

        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut mod_info = archive.by_name("modinfo.json")?;
        let mut contents = Vec::new();
        mod_info.read_to_end(&mut contents)?;
        Ok(contents)
    }

    /// Deletes a file asynchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn delete_file(&self, path_buf: &PathBuf) -> Result<(), FileError> {
        self.logger
            .log_default(&format!("Deleting file: {}", path_buf.display()));
        fs::remove_file(path_buf).await?;
        Ok(())
    }

    /// Deletes a file synchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn delete_file_sync(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Deleting file: {file_name}"));
        std::fs::remove_file(file_name)?;
        Ok(())
    }

    /// Checks if a file exists asynchronously.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to check.
    ///
    /// # Returns
    ///
    /// A `Result` containing `true` if the file exists, `false` if it does not, or an error.
    pub async fn file_exists(&self, file_name: &str) -> Result<bool, std::io::Error> {
        self.logger
            .log_default(&format!("Checking if file exists: {file_name}"));
        match fs::metadata(file_name).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Checks if a file exists synchronously.
    ///
    /// # Arguments
    /// * `directory` - a string slice representing the directory to search.
    ///
    /// # Returns
    ///  A `Result` containing a vector of strings or an error.
    pub async fn get_files_in_directory(
        &self, directory: &PathBuf,
    ) -> Result<Vec<String>, std::io::Error> {
        self.logger.log_default(&format!(
            "Getting files in directory: {}",
            directory.display()
        ));
        let mut files = vec![];
        let entries = fs::read_dir(directory).await?;
        let mut entries = ReadDirStream::new(entries);
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.to_str().unwrap().to_string();
            files.push(file_name);
        }
        Ok(files)
    }

    pub async fn read_mod_info_from_zips(
        &self, paths: Vec<PathBuf>,
    ) -> Result<Vec<Vec<u8>>, FileError> {
        let mut zips = Vec::with_capacity(paths.len());
        for path in paths {
            self.validate_path(&path).await?;
            let zip = self.read_mod_info_from_zip(&path)?;
            zips.push(zip);
        }
        Ok(zips)
    }

    async fn get_mod_info_with_paths(&self) -> Result<Vec<(Vec<u8>, PathBuf)>, FileError> {
        let mut mod_info = Vec::new();
        let entries = fs::read_dir(&self.base_path).await?;
        let mut entries = ReadDirStream::new(entries);

        while let Some(entry) = entries.next().await {
            let entry = entry?;
            let path = entry.path();
            if self.is_valid_mod_file(&path) {
                let zip = self.read_mod_info_from_zip(&path)?;
                mod_info.push((zip, path));
            }
        }
        Ok(mod_info)
    }

    pub async fn collect_mods(
        &self, filters: &Option<CliFlags>,
    ) -> Result<Vec<(ModInfo, PathBuf)>, FileError> {
        let default_flags = CliFlags::default();
        let option = filters.as_ref().unwrap_or(&default_flags);
        let mod_vec: Vec<(Vec<u8>, PathBuf)> = self.get_mod_info_with_paths().await?;

        let mods = mod_vec
            .into_iter()
            .filter_map(|(mod_slice, path)| {
                let mod_string = std::str::from_utf8(&mod_slice).ok()?;
                let mod_string = remove_trailing_comma(mod_string);
                let mod_info: ModInfo = serde_json::from_str(&mod_string.to_lowercase()).ok()?;
                Some((mod_info, path))
            })
            .filter(|(mod_info, _)| {
                if let Some(mod_) = &option.mod_ {
                    return mod_info
                        .modid
                        .as_ref()
                        .map(|id| id.contains(&mod_.to_lowercase()))
                        .unwrap_or(false);
                }
                if let Some(include) = &option.include {
                    return mod_info
                        .modid
                        .as_ref()
                        .map(|id| include.contains(&id.to_lowercase()))
                        .unwrap_or(false);
                }
                if let Some(exclude) = &option.exclude {
                    return mod_info
                        .modid
                        .as_ref()
                        .map(|id| !exclude.contains(&id.to_lowercase()))
                        .unwrap_or(true);
                }
                true
            })
            .collect();

        Ok(mods)
    }
}

fn remove_trailing_comma(json: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut in_escape = false;

    for (i, c) in json.chars().enumerate() {
        if in_escape {
            in_escape = false;
        } else if c == '\\' {
            in_escape = true;
        } else if c == '"' {
            in_string = !in_string;
        }

        if !in_string && c == ',' {
            if let Some(next_char) = json.chars().nth(i + 1) {
                if next_char.is_whitespace() {
                    if let Some(non_whitespace_char) =
                        json.chars().skip(i + 1).find(|&ch| !ch.is_whitespace())
                    {
                        if !"{[\"'\\w".contains(non_whitespace_char) {
                            continue;
                        }
                    }
                } else if !"{[\"'\\w".contains(next_char) {
                    continue;
                }
            }
        }

        result.push(c);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[tokio::test]
    async fn read_file_returns_correct_content() {
        let file_manager = FileManager::new(false);
        let test_content = b"test content";
        let temp_dir = tempdir().unwrap();
        let test_file_path = temp_dir.path().join("test_read_file.txt");

        let mut file = fs::File::create(&test_file_path).await.unwrap();

        println!("test_file_path: {test_file_path:?}");

        file.write_all(test_content).await.unwrap();

        let read_content = file_manager.read_file(&test_file_path).await.unwrap();
        assert_eq!(read_content, test_content);
    }

    #[tokio::test]
    async fn save_file_creates_file_with_correct_content() {
        // Create a temporary directory for test files
        let temp_dir = tempdir().unwrap();
        let test_file_path = temp_dir.path().join("test_save_file.txt");
        let test_content = b"test content";

        // Create a test file manager
        let file_manager = FileManager::new(false);

        // Save the test content
        file_manager
            .save_file(&test_file_path, test_content)
            .await
            .unwrap();

        // Verify file exists and has correct content
        let read_content = std::fs::read(&test_file_path).unwrap();
        assert_eq!(read_content, test_content);

        // Cleanup happens automatically when temp_dir is dropped
    }

    #[tokio::test]
    async fn delete_file_removes_file() {
        let file_manager = FileManager::new(false);
        let file_name = &PathBuf::from("test_delete_file.txt");
        let content = "Hello, World!".as_bytes();

        std::fs::write(file_name, content).unwrap();
        let result = file_manager.delete_file(file_name).await;
        assert!(result.is_ok());

        let file_exists = std::fs::metadata(file_name).is_ok();
        assert!(!file_exists);
    }

    #[tokio::test]
    async fn file_exists_returns_true_for_existing_file() {
        let file_manager = FileManager::new(false);
        let file_name = "test_file_exists.txt";
        let content = "Hello, World!".as_bytes();

        std::fs::write(file_name, content).unwrap();
        let exists = file_manager.file_exists(file_name).await.unwrap();
        assert!(exists);

        std::fs::remove_file(file_name).unwrap();
    }

    #[tokio::test]
    async fn file_exists_returns_false_for_non_existing_file() {
        let file_manager = FileManager::new(false);
        let file_name = "non_existing_file.txt";

        let exists = file_manager.file_exists(file_name).await.unwrap();
        assert!(!exists);
    }
}
