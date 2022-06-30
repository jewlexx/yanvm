use reqwest::Client;

use crate::install::download_binary;

mod consts;
mod install;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let version = (16, 15, 1);

    download_binary(&client, version, versions::Arch::X64).await?;

    Ok(())
}
