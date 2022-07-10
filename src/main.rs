use clap::{IntoApp, Parser};
use config::Config;
use installer::Installer;

mod args;
mod config;
mod consts;
mod helpers;
mod installer;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    // Initialize Global Config
    {
        let config = Config::init()?;
        *consts::CONFIG.lock() = config;
    }

    match args.command {
        None => {
            let config = consts::CONFIG.lock();
            if config.versions.is_empty() {
                println!(
                "No versions installed. Please run `yanvm install` to install a NodeJS version."
            );
            } else if config.current == None {
                println!(
                    "No current version set. Please run `yanvm set` to set a current version."
                );
            } else {
                args::Args::command().print_help()?;
            }

            return Ok(());
        }
        Some(command) => match command {
            args::Commands::Install { version_str } => {
                let version = match version_str.as_str() {
                    "latest" => Installer::latest_version().await?,
                    "lts" => Installer::lts_version().await?,
                    _ => anyhow::bail!("Unexpected version string."),
                };

                version
                    .download_binary(std::env::current_dir()?)
                    .await?
                    .unzip_file()
                    .await?;
            }
        },
    }

    Ok(())
}
