use reqwest::Client;

mod consts;
mod install;
mod versions;

use versions::index::NodeIndex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let index = versions::index::list_index(&client).await?;

    for i in index {
        println!("{}", i.version);
    }

    Ok(())
}
