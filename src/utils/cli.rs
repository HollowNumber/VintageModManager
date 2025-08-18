use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author = "Mikkel M.H Pedersen",
    version = "0.7.2",
    long_about = "A mod manager for the game Vintage Story.\nCreated by Mikkel M.H Pedersen.\nThis CLI tool helps you manage Vintage Story mods through three main commands:\n- download: Get mods from the official repository\n- export: Create shareable mod collections\n- update: Keep your mods up to date"
)]
pub struct Cli {
    #[clap(short, long, action=ArgAction::SetTrue)]
    /// Enable detailed logging output for troubleshooting
    pub verbose: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download mods from the official Vintage Story repository
    Download {
        #[clap(long)]
        /// Import mods using an encoded mod string (obtained from the export command)
        mod_string: Option<String>,

        #[clap(long, value_delimiter = ',')]
        /// Download multiple mods by their IDs or names (comma-separated)
        /// Example: --mods "worldedit,prospecting,bettertools"
        mods: Option<Vec<String>>,

        #[clap(long)]
        /// Download a single mod by its ID or name
        /// Example: --mod worldedit
        mod_: Option<String>,
    },

    /// Create shareable mod collections as encoded strings
    ///
    /// This command allows you to create encoded strings that can be shared with others to import specific mod collections.
    ///
    /// Default behavior is to export all mods.
    Export {
        #[clap(short, long)]
        /// List of mod IDs to exclude from the export (comma-separated)
        /// Example: -e "worldedit,prospecting"
        exclude: Option<Vec<String>>,

        #[clap(short, long)]
        /// List of specific mod IDs to include in the export (comma-separated)
        /// Example: -i "worldedit,prospecting"
        include: Option<Vec<String>>,

        #[clap(short, long)]
        /// Export only one specific mod by its ID
        /// Example: -m worldedit
        mod_: Option<String>,

        #[clap(long, action=ArgAction::SetTrue)]
        /// Select mods to export through an interactive menu
        interactive: Option<bool>,
    },

    /// Check for and install available mod updates
    Update {
        #[clap(short, long)]
        /// List of mod IDs to skip during update (comma-separated)
        ///
        /// Example: -e "worldedit,prospecting"
        exclude: Option<Vec<String>>,

        #[clap(short, long)]
        /// List of specific mod IDs to update (comma-separated)
        ///
        /// Example: -i "worldedit,prospecting"
        include: Option<Vec<String>>,

        #[clap(short, long)]
        /// Update only one specific mod by its ID
        ///
        /// Example: -m worldedit
        mod_: Option<String>,
    },

    /// Manage configuration settigns
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set the Vintage Story game installation path
    SetPath {
        /// Path to the Vintage Story installation directory
        path: PathBuf,
    },

    /// Show current configuration
    Show,

    /// Initialize configuration file with default values
    Init {
        #[clap(long)]
        /// Force overwrite existing config file
        force: bool,
    },

    /// Update version mappings from the API
    UpdateVersions {
        #[clap(long, action=ArgAction::SetTrue)]
        /// Show progress during update
        verbose: Option<bool>,
    },

    /// List all available game versions
    ListVersions,

    /// Reset configuration to defaults
    Reset {
        #[clap(long)]
        /// Confirm reset without prompting
        yes: bool,
    },

    /// Validate current configuration
    Validate,

    /// Set the current game version for compatibility filtering
    SetGameVersion {
        /// Game version string (e.g., "1.15.3")
        version: String,
    },
}

#[derive(Default)]
pub struct CliFlags {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub mod_: Option<String>,
}

#[derive(Default)]
pub struct DownloadFlags {
    pub mod_string: Option<String>,
    pub mods: Option<Vec<String>>,
    pub mod_: Option<String>,
}

pub trait IsAllNone {
    fn is_all_none(&self) -> bool;
}

impl IsAllNone for DownloadFlags {
    fn is_all_none(&self) -> bool {
        self.mod_string.is_none() && self.mods.is_none() && self.mod_.is_none()
    }
}

impl IsAllNone for CliFlags {
    fn is_all_none(&self) -> bool {
        self.exclude.is_none() && self.include.is_none() && self.mod_.is_none()
    }
}
