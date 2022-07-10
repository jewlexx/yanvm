use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Yet Another Node Version Manager", about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    Install {
        #[clap(
            name = "VERSION",
            help = "The version to install. [lts or latest act as variables for their respective versions]",
            default_value = "lts"
        )]
        version_str: String,
    },
}
