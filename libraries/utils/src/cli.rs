use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Mikkel M.H Pedersen", version = "0.1.0", long_about = None)]
pub struct CLI {
    #[clap(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ImportMods {
        /// The import string
        #[clap(short, long)]
        mod_list: String,
    },

    ExportMods {
        /// Whether to export the mods
        #[clap(short, long)]
        do_export: bool,
    },

    CheckForUpdates {
        /// The mod ID
        #[clap(short, long)]
        do_check: bool,
    },
}
