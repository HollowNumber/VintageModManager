use directories::BaseDirs;
use std::path::PathBuf;

const UNIX_PATH: &str = "/VintagestoryData/Mods/";

/// Get the configuration directory for the current user.
///
/// # Returns
///
/// A `String` representing the path to the configuration directory.
///
/// # Panics
///
/// This function will panic if the base directories cannot be determined or if the configuration directory cannot be converted to a string.
pub fn get_config_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("Could not get base directories");
    base_dirs.config_dir().to_path_buf()
}

/// Get the directory where Vintage Story mods are stored.
///
/// Does not check if the directory exists.
///
/// # Returns
///
/// A `String` representing the path to the Vintage Story mods directory.
pub fn get_vintage_mods_dir() -> Result<PathBuf, std::io::Error> {
    let config_dir = get_config_dir();

    let sys_path = if cfg!(unix) || cfg!(target_os = "macos") {
        PathBuf::from(UNIX_PATH)
    } else if cfg!(windows) {
        PathBuf::from("VintagestoryData").join("Mods")
    } else {
        panic!("Unsupported operating system");
    };

    let mods_dir = config_dir.join(sys_path);

    if !mods_dir.exists() {
        // as the mods dir is created by the game we just want to panic out if it doesn't exist
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Mods directory not found",
        ));
    }

    Ok(mods_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_dir() {
        let config_dir = get_config_dir();
        // Assert that the path
        assert!(!config_dir.to_str().unwrap().is_empty());
    }

    #[test]
    fn test_get_vintage_mods_dir() {
        let mods_dir = get_vintage_mods_dir();
        assert!(
            !mods_dir
                .expect("Path Not Found")
                .to_str()
                .unwrap()
                .is_empty()
        );
    }
}
