use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionMapping {
    pub tag_id: i64,
    pub version: String,
}

impl VersionMapping {
    pub fn new(tag_id: i64, version: String) -> Self {
        Self { tag_id, version }
    }
}

/// Struct to represent the configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the Vintage Story game installation directory
    pub game_path: Option<PathBuf>,

    /// Mapping of version tag IDs to version strings
    pub version_mapping: Vec<VersionMapping>,

    /// Current detected game version (auto-detected from assets/{version}.txt)
    pub detected_game_version: Option<String>,
}

impl Config {
    /// Creates a new `Config` instance with default values.
    pub fn new() -> Self {
        Self {
            game_path: None,
            version_mapping: Vec::new(),
            detected_game_version: None,
        }
    }

    /// Creates a `Config` instance with a specified game path.
    pub fn with_game_path(mut self, game_path: PathBuf) -> Self {
        self.game_path = Some(game_path);
        self
    }

    /// Adds a version mapping to the config.
    pub fn add_version_mapping(mut self, tag_id: i64, version: String) -> Self {
        self.version_mapping
            .push(VersionMapping::new(tag_id, version));
        self
    }

    /// Detects the game version from assets/version-{version}.txt file
    pub fn detect_game_version(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if let Some(game_path) = &self.game_path {
            let assets_dir = game_path.join("assets");

            if !assets_dir.exists() {
                return Ok(None);
            }

            // Look for version files in the assets directory
            if let Ok(entries) = fs::read_dir(&assets_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        // Look for files matching "version-{version}.txt" pattern
                        if file_name.starts_with("version-") && file_name.ends_with(".txt") {
                            // Extract version from filename: "version-1.20.3.txt" -> "1.20.3"
                            let version = file_name
                                .strip_prefix("version-")
                                .and_then(|s| s.strip_suffix(".txt"))
                                .map(|s| s.to_string());

                            if let Some(version) = version {
                                if self.looks_like_version(&version) {
                                    self.detected_game_version = Some(version.clone());
                                    return Ok(Some(version));
                                }
                            }
                        }
                    }
                }
            }

            // Alternative: try to read content of a generic version file if it exists
            // This is a fallback in case the naming convention changes
            let version_file = assets_dir.join("version.txt");
            if version_file.exists() {
                let version = fs::read_to_string(version_file)?.trim().to_string();
                if self.looks_like_version(&version) {
                    self.detected_game_version = Some(version.clone());
                    return Ok(Some(version));
                }
            }
        }

        Ok(None)
    }

    /// Check if a filename looks like a version number
    fn looks_like_version(&self, filename: &str) -> bool {
        let name_without_ext = filename.trim_end_matches(".txt");

        // Check if it matches version patterns like "1.15.3", "1.14.10-rc.1", etc.
        let version_patterns = [
            // Standard versions: 1.15.3, 1.14.10
            regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap(),
            // RC versions: 1.15.3-rc.1
            regex::Regex::new(r"^\d+\.\d+\.\d+-rc\.\d+$").unwrap(),
            // Dev versions: 1.15.0-dev.4
            regex::Regex::new(r"^\d+\.\d+\.\d+-dev\.\d+$").unwrap(),
        ];

        version_patterns
            .iter()
            .any(|pattern| pattern.is_match(name_without_ext))
    }

    /// Get the currently detected game version
    pub fn get_detected_game_version(&self) -> Option<&String> {
        self.detected_game_version.as_ref()
    }

    /// Get the tag ID for the detected game version
    pub fn get_detected_version_tag_id(&self) -> Option<i64> {
        if let Some(version) = &self.detected_game_version {
            self.get_tag_from_version(version)
        } else {
            None
        }
    }

    /// Check if the detected game version is compatible (has a mapping)
    pub fn is_detected_version_mapped(&self) -> bool {
        if let Some(version) = &self.detected_game_version {
            self.get_tag_from_version(version).is_some()
        } else {
            false
        }
    }

    // ... rest of the existing methods remain the same ...

    /// Loads configuration from a TOML file.
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Saves the configuration to a TOML file.
    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(file_path, toml_string)?;
        Ok(())
    }

    /// Gets the game installation path.
    pub fn get_game_path(&self) -> Option<&PathBuf> {
        self.game_path.as_ref()
    }

    /// Sets the game installation path.
    pub fn set_game_path(&mut self, path: PathBuf) {
        self.game_path = Some(path);
    }

    /// Gets a version string from a tag ID.
    pub fn get_version_from_tag(&self, tag_id: i64) -> Option<&String> {
        self.version_mapping
            .iter()
            .find(|mapping| mapping.tag_id == tag_id)
            .map(|mapping| &mapping.version)
    }

    /// Gets a tag ID from a version string.
    pub fn get_tag_from_version(&self, version: &str) -> Option<i64> {
        self.version_mapping
            .iter()
            .find(|mapping| mapping.version == version)
            .map(|mapping| mapping.tag_id)
    }

    /// Updates the version mapping with new data.
    pub fn update_version_mapping(&mut self, mappings: Vec<VersionMapping>) {
        self.version_mapping = mappings;
    }

    /// Adds or updates a single version mapping.
    pub fn set_version_mapping(&mut self, tag_id: i64, version: String) {
        if let Some(existing) = self
            .version_mapping
            .iter_mut()
            .find(|mapping| mapping.tag_id == tag_id)
        {
            existing.version = version;
        } else {
            self.version_mapping
                .push(VersionMapping::new(tag_id, version));
        }
    }

    /// Gets all available versions as a sorted vector.
    pub fn get_all_versions(&self) -> Vec<&String> {
        let mut versions: Vec<_> = self
            .version_mapping
            .iter()
            .map(|mapping| &mapping.version)
            .collect();
        versions.sort();
        versions
    }

    /// Checks if a version mapping exists.
    pub fn has_version_mapping(&self) -> bool {
        !self.version_mapping.is_empty()
    }

    /// Gets all version mappings.
    pub fn get_all_mappings(&self) -> &[VersionMapping] {
        &self.version_mapping
    }

    /// Removes a version mapping by tag ID.
    pub fn remove_version_mapping(&mut self, tag_id: i64) -> bool {
        if let Some(pos) = self
            .version_mapping
            .iter()
            .position(|mapping| mapping.tag_id == tag_id)
        {
            self.version_mapping.remove(pos);
            true
        } else {
            false
        }
    }

    /// Checks if a tag ID exists.
    pub fn has_tag_id(&self, tag_id: i64) -> bool {
        self.version_mapping
            .iter()
            .any(|mapping| mapping.tag_id == tag_id)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
