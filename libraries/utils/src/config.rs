use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};

/// Struct to represent the configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// A hashmap to store configuration settings as key-value pairs.
    settings: HashMap<String, String>,
}

impl Config {
    /// Creates a new `Config` instance with an empty settings hashmap.
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }

    /// Loads configuration from a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the configuration file.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
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

    /// Creates a `Config` instance from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json_str` - A string slice that holds the JSON representation of the configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Config` instance or a `serde_json::Error`.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let settings: HashMap<String, String> = serde_json::from_str(json_str)?;
        Ok(Self { settings })
    }

    /// Saves the configuration to a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the configuration file.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = fs::File::create(file_path)?;
        for (key, value) in &self.settings {
            writeln!(file, "{}={}", key, value)?;
        }
        Ok(())
    }

    /// Saves a serializable data structure as a binary JSON file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the binary JSON file.
    /// * `data` - A reference to the data structure to be serialized and saved.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    pub fn save_json_as_binary<T: Serialize>(file_path: &str, data: &T) -> io::Result<()> {
        let json_string = serde_json::to_string(data)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    /// Gets a configuration value.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key of the configuration setting.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the value if it exists, or `None` if it does not.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    /// Sets a configuration value.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key of the configuration setting.
    /// * `value` - A `String` representing the value to be set.
    pub fn set(&mut self, key: &str, value: String) {
        self.settings.insert(key.to_string(), value);
    }
}
