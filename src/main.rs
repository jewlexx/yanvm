use config::Config;
use versions::index::list_index;

mod config;
mod consts;
mod installer;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;
    if config.current == None {
        println!("No current version set");
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
