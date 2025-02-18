use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Mikkel M.H Pedersen", version = "0.1.0", long_about = None)]
pub struct CLI {
    #[clap(short, long, action=ArgAction::SetTrue)]
    pub verbose: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Imports mods from a `mod string` to the VintageStoryData/mods directory
    Import {
        #[clap(short, long)]
        /// The `mod string` is the mod name and version separated by a | and each mod separated by a `;` and afterward encoded to base64
        mod_string: Option<String>,

        #[clap(long, action=ArgAction::SetTrue, help="Multi thread the download of mods, currently not implemented"
        )]
        /// Multi thread the download of mods
        /// This will download multiple mods at the same time
        multi_thread: Option<bool>,
    },

    /// Exports mods from the VintageStoryData/mods directory to a `mod string`.
    /// The `mod string` contains mod names and versions separated by `|` and each mod separated by `;`, then encoded to Base64.
    /// It can be used to import mods on another computer.
    Export {
        #[clap(short, long, action=ArgAction::SetTrue)]
        export: Option<bool>,
    },

    Update {
        #[clap(short, long, action=ArgAction::SetTrue)]
        /// Checks for mod updates in the VintageStoryData/mods directory.
        check: Option<bool>,

        #[clap(short, long, action=ArgAction::SetTrue)]
        /// Updates the mods in the VintageStoryData/mods directory
        update: Option<bool>,
    },
}
