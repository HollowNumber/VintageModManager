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
    /// Imports mods from to the mod folder.
    Download {
        #[clap(long)]
        /// The <mod string> to import, gotten from the export command
        mod_string: Option<String>,

        #[clap(long)]
        /// mods to download, can be either a <mod id> or a mod name
        mods: Option<Vec<String>>,

        #[clap(long, name = "mod")]
        /// The <mod id> or <name> of the mod to download
        mod_: Option<String>,
    },

    /// Exports mods from the mod folder to a shareable string.
    Export {
        #[clap(short, long)]
        /// Exports the <mod ids> in the mod folder that are not in the <exclude> list
        exclude: Option<Vec<String>>,

        #[clap(short, long)]
        /// Exports the specified <mod ids> in the mod folder
        include: Option<Vec<String>>,

        #[clap(short, long, name = "mod")]
        /// Only exports the <mod id> specified
        mod_: Option<String>,
    },

    /// Updates mods in the mod folder.
    Update {
        #[clap(short, long)]
        /// Updates the <mod ids> in the mod folder that are not in the <exclude> list, separated by a comma
        exclude: Option<Vec<String>>,

        #[clap(short, long)]
        /// Updates the specified <mod ids> in the mod folder, separated by a comma
        include: Option<Vec<String>>,

        #[clap(short, long, name = "mod")]
        /// Only updates the <mod id> specified
        mod_: Option<String>,
    },
}

pub struct CliOptions {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub mod_: Option<String>,
}

pub struct DownloadOptions {
    pub mod_string: Option<String>,
    pub mods: Option<Vec<String>>,
    pub mod_: Option<String>,
}

impl Default for CliOptions {
    fn default() -> Self {
        CliOptions {
            exclude: None,
            include: None,
            mod_: None,
        }
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        DownloadOptions {
            mod_string: None,
            mods: None,
            mod_: None,
        }
    }
}
