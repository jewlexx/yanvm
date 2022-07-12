use clap::{IntoApp, Parser};
use config::Config;
use installer::Installer;

mod args;
mod config;
mod consts;
mod helpers;
mod installer;
mod versions;

#[macro_use]
mod macros;

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

    // let cwd = std::env::current_dir().unwrap();
    // let og = cwd.join("node-v16.16.0-win-x64");
    // let to = cwd.join("v16-node");

    // std::os::windows::fs::symlink_dir(og, to).unwrap();

    Ok(())
}
