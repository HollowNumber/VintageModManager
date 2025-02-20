use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Mikkel M.H Pedersen", version = "0.5.3", long_about = None)]
pub struct CLI {
    #[clap(short, long, action=ArgAction::SetTrue)]
    /// Enable verbose output
    pub verbose: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Imports mods from a `mod string` to the VintageStoryData/mods directory
    Download {
        #[clap(
            long,
            help = "The mod string to import, gotten from the export command"
        )]
        mod_string: Option<String>,

        #[clap(
            long,
            help = "mods to download, can be either a mod id or a mod name",
            value_delimiter = ' '
        )]
        mods: Option<Vec<String>>,

        #[clap(long, help = "The mod id or name of the mod to download", name = "mod")]
        mod_: Option<String>,

        #[clap(long, action=ArgAction::SetTrue, help="Multi thread the download of mods, currently not implemented"
        )]
        multi_thread: Option<bool>,
    },

    /// Exports mods from the mod folder to a shareable string.
    Export {
        #[clap(short, long, action=ArgAction::SetTrue)]
        /// Exports all mods in the mod folder, default behaviour.
        all: Option<bool>,

        #[clap(long, value_delimiter = ' ')]
        /// Exports the `mod ids` in the mod folder that are not in the `exclude` list
        exclude: Option<Vec<String>>,

        #[clap(long, value_delimiter = ' ')]
        /// Exports the specified `mod ids` in the mod folder
        include: Option<Vec<String>>,

        #[clap(long, name = "mod")]
        /// Only exports the `mod id` specified
        mod_: Option<String>,
    },

    /// Updates mods in the mod folder.
    Update {
        #[clap(short, long, action=ArgAction::SetTrue)]
        /// Updates all mods in the mod folder, default behaviour.
        all: Option<bool>,

        #[clap(long, value_delimiter = ' ')]
        /// Updates the `mod ids` in the mod folder that are not in the `exclude` list
        exclude: Option<Vec<String>>,

        #[clap(long, value_delimiter = ' ')]
        /// Updates the specified `mod ids` in the mod folder
        include: Option<Vec<String>>,

        #[clap(long, name = "mod")]
        /// Only updates the `mod id` specified
        mod_: Option<String>,
    },
}

pub struct CliOptions {
    pub all: Option<bool>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub mod_: Option<String>,
}

pub struct DownloadOptions {
    pub mod_string: Option<String>,
    pub mods: Option<Vec<String>>,
    pub mod_: Option<String>,
    pub multi_thread: Option<bool>,
}

impl Default for CliOptions {
    fn default() -> Self {
        CliOptions {
            all: None,
            exclude: None,
            include: None,
            mod_: None,
        }
    }
}
