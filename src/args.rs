use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Yet Another Node Version Manager", about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[clap(long, help = "Force running as admin")]
    pub admin: bool,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    Install {
        #[clap(
            name = "VERSION",
            help = "The version to install. Possible Formats: lts, latest, vXX.XX.XX",
            default_value = "lts"
        )]
        version_str: String,
    },
}
