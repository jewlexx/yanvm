use clap::Parser;

#[derive(Parser)]
#[clap(name = "Yet Another Node Version Manager", about)]
pub struct Args {
    pub command: Option<String>,
}