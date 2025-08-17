use crate::api::VintageApiHandler;
use crate::config::{Config, VersionMapping};
use crate::utils::terminal::Terminal;
use crate::utils::{LogLevel, Logger};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Config file not found: {0}")]
    NotFound(String),
    #[error("Invalid game path: {0}")]
    InvalidGamePath(String),
    #[error("API error: {0}")]
    Api(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
    logger: Logger,
}

impl ConfigManager {
    /// Create a new ConfigManager
    pub fn new(verbose: bool) -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;
        let logger = Logger::new("ConfigManager".to_string(), LogLevel::Info, None, verbose);

        let mut config = if config_path.exists() {
            Self::load_config_from_file(&config_path)?
        } else {
            Config::new()
        };

        // Auto-detect game version if path is set but version isn't detected yet
        if config.get_game_path().is_some() && config.get_detected_game_version().is_none() {
            if let Err(e) = config.detect_game_version() {
                // Log but don't fail - version detection is optional
                eprintln!("Warning: Could not detect game version: {}", e);
            }
        }

        Ok(Self {
            config_path,
            config,
            logger,
        })
    }

    /// Get the standard config file path
    fn get_config_path() -> Result<PathBuf, ConfigError> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "mikkelmh", "vintage-story-mod-manager") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(config_dir)?;
            Ok(config_dir.join("config.toml"))
        } else {
            Err(ConfigError::NotFound(
                "Could not determine config directory".to_string(),
            ))
        }
    }

    /// Load config from file
    fn load_config_from_file(path: &Path) -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save current config to file
    pub fn save(&self) -> Result<(), ConfigError> {
        let toml_string = toml::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, toml_string)?;
        self.logger.log_default("Configuration saved");
        Ok(())
    }

    /// Initialize config file with defaults
    pub fn init(&mut self, force: bool) -> Result<(), ConfigError> {
        if self.config_path.exists() && !force {
            return Err(ConfigError::NotFound(format!(
                "Config file already exists at {}. Use --force to overwrite.",
                self.config_path.display()
            )));
        }

        self.config = Config::new();

        // Try to auto-detect game path
        if let Some(game_path) = self.try_detect_game_path() {
            self.config.set_game_path(game_path);

            // Try to detect game version
            if let Ok(Some(version)) = self.config.detect_game_version() {
                println!(
                    "Auto-detected game path: {} (version: {})",
                    self.config.get_game_path().unwrap().display(),
                    version
                );
            } else {
                println!(
                    "Auto-detected game path: {}",
                    self.config.get_game_path().unwrap().display()
                );
            }
        }

        self.save()?;
        println!(
            "Configuration initialized at: {}",
            self.config_path.display()
        );
        Ok(())
    }

    /// Try to auto-detect Vintage Story installation
    fn try_detect_game_path(&self) -> Option<PathBuf> {
        let possible_paths = vec![
            // Windows
            PathBuf::from(r"C:\Program Files\Vintage Story"),
            PathBuf::from(r"C:\Program Files (x86)\Vintage Story"),
            // Linux
            PathBuf::from("/opt/vintagestory"),
            PathBuf::from(format!(
                "{}/.local/share/VintageStory",
                std::env::var("HOME").unwrap_or_default()
            )),
            // macOS
            PathBuf::from("/Applications/Vintage Story.app"),
        ];

        for path in possible_paths {
            if path.exists() && self.validate_game_path(&path) {
                return Some(path);
            }
        }
        None
    }

    /// Set game installation path and auto-detect version
    pub fn set_game_path(&mut self, path: PathBuf) -> Result<(), ConfigError> {
        if !path.exists() {
            return Err(ConfigError::InvalidGamePath(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }

        if !self.validate_game_path(&path) {
            return Err(ConfigError::InvalidGamePath(format!(
                "Path does not appear to be a valid Vintage Story installation: {}",
                path.display()
            )));
        }

        self.config.set_game_path(path.clone());

        // Auto-detect game version
        match self.config.detect_game_version() {
            Ok(Some(version)) => {
                println!("Game path set to: {}", path.display());
                println!("Detected game version: {}", version);

                // Check if we have a mapping for this version
                if self.config.is_detected_version_mapped() {
                    println!("Version mapping available");
                } else {
                    println!(
                        "No version mapping for {}. Run 'config update-versions' to fetch mappings.",
                        version
                    );
                }
            }
            Ok(None) => {
                println!("Game path set to: {}", path.display());
                println!("Could not auto-detect game version from assets directory");
            }
            Err(e) => {
                println!("Game path set to: {}", path.display());
                println!("Error detecting game version: {}", e);
            }
        }

        self.save()?;
        Ok(())
    }

    /// Validate that a path contains a Vintage Story installation
    fn validate_game_path(&self, path: &Path) -> bool {
        // Look for key Vintage Story files/directories
        let indicators = vec![
            "assets",
            "Lib",
            "VintageStory.exe", // Windows
            "VintageStory",     // Linux
            "Vintagestory.app", // macOS (alternative)
        ];

        indicators
            .iter()
            .any(|&indicator| path.join(indicator).exists())
    }

    /// Update version mappings from API
    pub async fn update_version_mappings(&mut self, verbose: bool) -> Result<(), ConfigError> {
        self.logger
            .log_default("Fetching version mappings from API...");

        let api = VintageApiHandler::new(verbose);

        // Fetch version mappings using the new method
        let version_mappings = api.fetch_game_versions().await?;

        self.config.update_version_mapping(version_mappings);
        self.save()?;

        println!(
            "Version mappings updated successfully ({} versions)",
            self.config.get_all_mappings().len()
        );

        // Check if our detected version now has a mapping
        if let Some(version) = self.config.get_detected_game_version() {
            if self.config.is_detected_version_mapped() {
                println!("Mapping found for detected version: {}", version);
            } else {
                println!("No mapping found for detected version: {}", version);
            }
        }

        Ok(())
    }

    /// Fetch game versions from API
    async fn fetch_game_versions(&self, api: &VintageApiHandler) -> Result<String, ConfigError> {
        // This should fetch the gameversions endpoint specifically
        // For now, using the existing fetch_mods as a placeholder
        // You might need to add a specific method to VintageApiHandler for this
        let response = api.fetch_mods().await?;
        Ok(response)
    }

    /// Parse API response to extract version mappings
    fn parse_api_versions(&self, api_response: &str) -> Result<Vec<VersionMapping>, ConfigError> {
        // Parse the JSON response that contains gameversions array
        let json_value: serde_json::Value = serde_json::from_str(api_response)?;

        let mut version_mappings = Vec::new();

        // Look for gameversions array in the response
        if let Some(game_versions) = json_value.get("gameversions").and_then(|v| v.as_array()) {
            for version_obj in game_versions {
                if let (Some(tag_id), Some(name)) = (
                    version_obj.get("tagid").and_then(|v| v.as_i64()),
                    version_obj.get("name").and_then(|v| v.as_str()),
                ) {
                    version_mappings.push(VersionMapping::new(tag_id, name.to_string()));
                }
            }
        }

        if version_mappings.is_empty() {
            // Fallback to some default mappings if parsing fails
            self.logger
                .log_default("Warning: Could not parse version mappings from API, using defaults");
            version_mappings = vec![
                VersionMapping::new(-281539401465857, "1.15.3-rc.1".to_string()),
                VersionMapping::new(-281539401285631, "1.15.0".to_string()),
                VersionMapping::new(-281535106973695, "1.14.10".to_string()),
            ];
        }

        Ok(version_mappings)
    }

    /// Show current configuration with detected version
    pub fn show(&self) {
        println!("Configuration file: {}", self.config_path.display());
        println!();

        if let Some(game_path) = self.config.get_game_path() {
            println!("Game path: {}", game_path.display());

            if let Some(version) = self.config.get_detected_game_version() {
                println!("Detected version: {}", version);

                if let Some(tag_id) = self.config.get_detected_version_tag_id() {
                    println!("Version tag ID: {}", tag_id);
                } else {
                    println!("Version tag ID: No mapping found");
                }
            } else {
                println!("Detected version: Could not detect");
            }
        } else {
            println!("Game path: Not set");
        }

        println!(
            "Version mappings: {} entries",
            self.config.get_all_mappings().len()
        );

        if !self.config.get_all_mappings().is_empty() {
            println!("\nAvailable game versions:");
            let versions = self.config.get_all_versions();
            for version in versions.iter().take(10) {
                // Show first 10
                let indicator = if Some(version.as_str())
                    == self.config.get_detected_game_version().map(|s| s.as_str())
                {
                    " (detected)"
                } else {
                    ""
                };
                println!("  - {}{}", version, indicator);
            }
            if versions.len() > 10 {
                println!("  ... and {} more", versions.len() - 10);
            }
        }
    }

    /// List all available game versions
    pub fn list_versions(&self) {
        let versions = self.config.get_all_versions();

        if versions.is_empty() {
            println!("No version mappings available. Run 'config update-versions' to fetch them.");
            return;
        }

        println!("Available game versions ({} total):", versions.len());
        for version in versions {
            if let Some(tag_id) = self.config.get_tag_from_version(version) {
                let indicator = if Some(version.as_str())
                    == self.config.get_detected_game_version().map(|s| s.as_str())
                {
                    " (detected)"
                } else {
                    ""
                };
                println!("  {} (tag: {}){}", version, tag_id, indicator);
            }
        }
    }

    /// Refresh detected game version
    pub fn refresh_detected_version(&mut self) -> Result<(), ConfigError> {
        match self.config.detect_game_version() {
            Ok(Some(version)) => {
                println!("Detected game version: {}", version);
                if self.config.is_detected_version_mapped() {
                    println!("Version mapping available");
                } else {
                    println!("⚠️  No version mapping available for this version");
                }
                self.save()?;
                Ok(())
            }
            Ok(None) => {
                println!("Could not detect game version from assets directory");
                Ok(())
            }
            Err(e) => Err(ConfigError::NotFound(format!(
                "Error detecting version: {}",
                e
            ))),
        }
    }

    /// Reset configuration
    pub fn reset(&mut self, confirmed: bool) -> Result<(), ConfigError> {
        if !confirmed && !Terminal::confirm("This will reset all configuration. Continue?") {
            println!("Reset cancelled.");
            return Ok(());
        }

        self.config = Config::new();
        self.save()?;
        println!("Configuration reset to defaults.");
        Ok(())
    }

    /// Validate current configuration including version detection
    pub fn validate(&self) -> Result<(), ConfigError> {
        println!("Validating configuration...");

        // Check game path
        if let Some(game_path) = self.config.get_game_path() {
            if !game_path.exists() {
                println!("Game path does not exist: {}", game_path.display());
            } else if !self.validate_game_path(game_path) {
                println!("Game path is not a valid Vintage Story installation");
            } else {
                println!("Game path is valid");

                // Check version detection
                if let Some(version) = self.config.get_detected_game_version() {
                    println!("Game version detected: {}", version);

                    if self.config.is_detected_version_mapped() {
                        println!("Version mapping available");
                    } else {
                        println!("No version mapping available for detected version");
                    }
                } else {
                    println!("Could not detect game version from assets directory");
                }
            }
        } else {
            println!("Game path not set");
        }

        // Check version mappings
        if self.config.has_version_mapping() {
            println!(
                "Version mappings available ({} entries)",
                self.config.get_all_mappings().len()
            );
        } else {
            println!("No version mappings available");
        }

        Ok(())
    }

    /// Get current config (read-only access)
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get tag ID for a version string
    pub fn get_tag_for_version(&self, version: &str) -> Option<i64> {
        self.config.get_tag_from_version(version)
    }

    /// Get the detected game version for filtering
    pub fn get_detected_game_version(&self) -> Option<&String> {
        self.config.get_detected_game_version()
    }

    /// Get the tag ID for the detected game version
    pub fn get_detected_version_tag_id(&self) -> Option<i64> {
        self.config.get_detected_version_tag_id()
    }

    /// Check if auto-detected version filtering should be used
    pub fn should_use_version_filtering(&self) -> bool {
        self.config.get_detected_game_version().is_some()
            && self.config.is_detected_version_mapped()
    }
}
