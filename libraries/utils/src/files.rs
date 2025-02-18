use crate::{get_vintage_mods_dir, LogLevel, Logger};
use bytes::Bytes;
use std::io::Read;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;
use zip::ZipArchive;

/// Struct to manage file operations with logging.
pub struct FileManager {
    /// Logger instance for logging file operations.
    logger: Logger,
}

impl FileManager {
    /// Creates a new `FileManager` instance.
    ///
    /// # Returns
    ///
    /// A new `FileManager` instance with a default logger.
    pub fn new(verbose: bool) -> Self {
        let logger = Logger::new("FileManager".to_string(), LogLevel::Info, None, verbose);
        Self { logger }
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
    pub async fn save_file(&self, file_name: &str, bytes: Bytes) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Saving file: {}", file_name));
        let mut file = fs::File::create(file_name).await?;
        file.write_all(&bytes).await?;
        Ok(())
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
    pub fn save_file_sync(&self, file_name: &str, bytes: Bytes) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Saving file: {}", file_name));
        let mut file = std::fs::File::create(file_name)?;
        std::io::Write::write_all(&mut file, &bytes)?;
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
    pub async fn read_file(&self, file_name: &str) -> Result<Bytes, std::io::Error> {
        self.logger
            .log_default(&format!("Reading file: {}", file_name));
        let mut file = fs::File::open(file_name).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        Ok(Bytes::from(contents))
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
    pub fn read_file_sync(&self, file_name: &str) -> Result<Bytes, std::io::Error> {
        self.logger
            .log_default(&format!("Reading file: {}", file_name));
        let mut file = std::fs::File::open(file_name)?;
        let mut contents = Vec::new();
        Read::read_to_end(&mut file, &mut contents)?;
        Ok(Bytes::from(contents))
    }

    /// Reads the `modinfo.json` file from a zip archive.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the zip file to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of `modinfo.json` as a `Vec<u8>` or an error.
    pub fn read_modinfo_from_zip(&self, file_name: &str) -> Result<Vec<u8>, std::io::Error> {
        self.logger
            .log_default(&format!("Reading zip file: {}", file_name));
        let file = std::fs::File::open(file_name)?;
        let mut archive = ZipArchive::new(file)?;
        // Look for the modinfo.json file
        let mut modinfo = archive.by_name("modinfo.json")?;
        let mut contents = Vec::new();
        modinfo.read_to_end(&mut contents)?;
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
    pub async fn delete_file(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Deleting file: {}", file_name));
        fs::remove_file(file_name).await?;
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
            .log_default(&format!("Deleting file: {}", file_name));
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
            .log_default(&format!("Checking if file exists: {}", file_name));
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
        &self,
        directory: &str,
    ) -> Result<Vec<String>, std::io::Error> {
        self.logger
            .log_default(&format!("Getting files in directory: {}", directory));
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

    pub async fn read_modinfo_from_zips(
        &self,
        paths: Vec<String>,
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let mut zips = vec![];
        for path in paths {
            let zip = self.read_modinfo_from_zip(&path)?;
            zips.push(zip);
        }
        Ok(zips)
    }

    pub async fn get_modinfo_from_mods_folder(&self) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let folder = get_vintage_mods_dir();
        let files = self.get_files_in_directory(&folder).await?;
        let zips = self.read_modinfo_from_zips(files).await?;
        Ok(zips)
    }
}
