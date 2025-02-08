use directories::BaseDirs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
}

/// Get the current system time as a UNIX timestamp.
pub fn get_system_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/*/// Get the value of an environment variable.
pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}*/

/// Get basic system information.
pub fn get_system_info() -> SystemInfo {
    let os = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();
    SystemInfo { os, arch }
}

pub enum SystemType {
    Windows,
    Linux,
    MacOS,
    Other,
}

pub fn get_config_dir() -> String {
    let base_dirs = BaseDirs::new().expect("Could not get base directories");
    let config_dir = base_dirs
        .config_dir()
        .to_str()
        .expect("Could not convert config dir to string");
    config_dir.to_string()
}
