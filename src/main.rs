use versions::index::list_index;

mod consts;
mod installer;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let index = list_index().await?;

    dialoguer::Select::new()
        .with_prompt("Select a version")
        .default(0)
        .items(&index)
        .interact()
        .unwrap();

    Ok(())
}
