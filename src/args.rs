use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Yet Another Node Version Manager", about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    Install { version_str: Option<String> },
}
