use clap::{IntoApp, Parser};
use config::Config;
use installer::Installer;
use versions::index::list_index;

mod args;
mod config;
mod consts;
mod helpers;
mod installer;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let config = Config::init()?;

    if args.command == None {
        if config.versions.is_empty() {
            println!(
                "No versions installed. Please run `yanvm install` to install a NodeJS version."
            );
        } else if config.current == None {
            println!("No current version set. Please run `yanvm set` to set a current version.");
        } else {
            args::Args::command().print_help()?;
        }

        return Ok(());
    }

    let version = Installer::latest_version().await?;

    version.download_binary(std::env::current_dir()?).await?;

    let index = list_index().await?;

    dialoguer::Select::new()
        .with_prompt("Select a version")
        .default(0)
        .items(&index)
        .interact()?;

    Ok(())
}
