use crate::{LogLevel, Logger};
use bytes::Bytes;
use std::io::Read;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

pub struct FileManager {
    logger: Logger,
}

impl FileManager {
    pub fn new() -> Self {
        let logger = Logger::new("FileManager".to_string(), LogLevel::Info);
        Self { logger }
    }

    pub async fn save_file(&self, file_name: &str, bytes: Bytes) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Saving file: {}", file_name));
        let mut file = fs::File::create(file_name).await?;
        file.write_all(&bytes).await?;
        Ok(())
    }

    pub fn save_file_sync(&self, file_name: &str, bytes: Bytes) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Saving file: {}", file_name));
        let mut file = std::fs::File::create(file_name)?;
        std::io::Write::write_all(&mut file, &bytes)?;
        Ok(())
    }

    pub async fn read_file(&self, file_name: &str) -> Result<Bytes, std::io::Error> {
        self.logger
            .log_default(&format!("Reading file: {}", file_name));
        let mut file = fs::File::open(file_name).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        Ok(Bytes::from(contents))
    }

    pub fn read_file_sync(&self, file_name: &str) -> Result<Bytes, std::io::Error> {
        self.logger
            .log_default(&format!("Reading file: {}", file_name));
        let mut file = std::fs::File::open(file_name)?;
        let mut contents = Vec::new();
        Read::read_to_end(&mut file, &mut contents)?;
        Ok(Bytes::from(contents))
    }

    pub fn read_modinfo_from_zip(&self, file_name: &str) -> Result<Vec<u8>, std::io::Error> {
        self.logger
            .log_default(&format!("Reading zip file: {}", file_name));
        let mut file = std::fs::File::open(file_name)?;
        let mut archive = ZipArchive::new(file)?;
        // Look for the modinfo.json file
        let mut modinfo = archive.by_name("modinfo.json")?;
        let mut contents = Vec::new();
        modinfo.read_to_end(&mut contents)?;
        Ok(contents)
    }

    pub async fn delete_file(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Deleting file: {}", file_name));
        fs::remove_file(file_name).await?;
        Ok(())
    }

    pub fn delete_file_sync(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.logger
            .log_default(&format!("Deleting file: {}", file_name));
        std::fs::remove_file(file_name)?;
        Ok(())
    }

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
}
