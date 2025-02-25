use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Mikkel M.H Pedersen", version = "0.6.0", long_about = None)]
pub struct Cli {
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
        /// The <mod string> to import, gotten from the export command.
        mod_string: Option<String>,

        #[clap(long, value_delimiter = ',')]
        /// Mods to download, can be either a <mod id> or a mod name.
        mods: Option<Vec<String>>,

        #[clap(long, name = "mod")]
        /// The <mod id> or <name> of the mod to download.
        mod_: Option<String>,
    },

    /// Exports mods from the mod folder to a shareable string.
    Export {
        #[clap(short, long)]
        /// Exports the <mod ids> in the mod folder that are not in the <exclude> list.
        exclude: Option<Vec<String>>,

        #[clap(short, long)]
        /// Exports the specified <mod ids> in the mod folder
        include: Option<Vec<String>>,

        #[clap(short, long, name = "mod")]
        /// Only exports the <mod id> specified
        mod_: Option<String>,

        #[clap(long, action=ArgAction::SetTrue)]
        interactive: Option<bool>,
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
