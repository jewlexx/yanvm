use clap::Parser;

#[derive(Parser)]
#[clap(name = "Yet Another Node Version Manager", about)]
pub struct Args {
    pub command: Option<Commands>,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[clap(subcommand)]
    Install,
}
