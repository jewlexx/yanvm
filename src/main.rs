use reqwest::Client;

mod consts;
mod install;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let index = versions::index::list_index(&client).await?;

    dialoguer::Select::new()
        .with_prompt("Select a version")
        .default(0)
        .items(&index)
        .interact()
        .unwrap();

    Ok(())
}
