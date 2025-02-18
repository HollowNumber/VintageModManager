use directories::BaseDirs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const WINDOWS_PATH: &str = "\\VintagestoryData\\Mods\\";
const UNIX_PATH: &str = "/VintagestoryData/Mods/";

/// Struct to hold basic system information.
#[derive(Debug)]
pub struct SystemInfo {
    /// Operating system name.
    pub os: String,
    /// System architecture.
    pub arch: String,
}

/// Get the current system time as a UNIX timestamp.
///
/// # Returns
///
/// A `u64` representing the number of seconds since the UNIX epoch.
pub fn get_system_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// Get basic system information.
///
/// # Returns
///
/// A `SystemInfo` struct containing the operating system name and system architecture.
#[deprecated(note = "Use cfg! macro instead")]
pub fn get_system_info() -> SystemInfo {
    let os = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();
    SystemInfo { os, arch }
}

/// Enum to represent different system types.
pub enum SystemType {
    Windows,
    Linux,
    MacOS,
    Other,
}

/// Get the configuration directory for the current user.
///
/// # Returns
///
/// A `String` representing the path to the configuration directory.
///
/// # Panics
///
/// This function will panic if the base directories cannot be determined or if the configuration directory cannot be converted to a string.
pub fn get_config_dir() -> String {
    let base_dirs = BaseDirs::new().expect("Could not get base directories");
    let config_dir = base_dirs
        .config_dir()
        .to_str()
        .expect("Could not convert config dir to string");
    config_dir.to_string()
}

pub fn get_vintage_mods_dir() -> String {
    let config_dir = get_config_dir();

    let sys_folder = if cfg!(unix) {
        UNIX_PATH
    } else if cfg!(windows) {
        WINDOWS_PATH
    } else if cfg!(macos) {
        UNIX_PATH
    } else {
        panic!("Unsupported operating system");
    };

    let vintage_mods_dir = format!("{}{}", config_dir, sys_folder);
    vintage_mods_dir
}
