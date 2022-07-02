use clap::{IntoApp, Parser};
use config::Config;
use versions::index::list_index;

mod args;
mod config;
mod consts;
mod installer;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let config = Config::init()?;

    if config.versions == vec![] {
        println!("No versions installed. Please run `yanvm install` to install a NodeJS version.");
        return Ok(());
    }

    if config.current == None {
        println!("No current version set. Please run `yanvm set` to set a current version.");
        return Ok(());
    }

    if args.command == None {
        args::Args::command().print_help()?;
        return Ok(());
    }

    let index = list_index().await?;

    dialoguer::Select::new()
        .with_prompt("Select a version")
        .default(0)
        .items(&index)
        .interact()
        .unwrap();

    Ok(())
}
