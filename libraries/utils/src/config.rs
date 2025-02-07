use std::fs;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    settings: HashMap<String, String>,
}


impl Config {
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }

    /// Load configuration from a file.
    pub fn load_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = fs::File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.settings = contents
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();
        Ok(())
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let settings: HashMap<String, String> = serde_json::from_str(json_str)?;
        Ok(Self { settings })
    }

    /// Save configuration to a file.
    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = fs::File::create(file_path)?;
        for (key, value) in &self.settings {
            writeln!(file, "{}={}", key, value)?;
        }
        Ok(())
    }

    pub fn save_json_as_binary<T: Serialize>(file_path: &str, data: &T) -> io::Result<()> {
        let json_string = serde_json::to_string(data)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    /// Get a configuration value.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    /// Set a configuration value.
    pub fn set(&mut self, key: &str, value: String) {
        self.settings.insert(key.to_string(), value);
    }
}
