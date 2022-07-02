use installer::Installer;
use versions::index::LtsUnion;

mod consts;
mod installer;
mod versions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let lts_version = Installer::lts_version().await?;

    println!("{}", lts_version.get_installer_link());

    // dialoguer::Select::new()
    //     .with_prompt("Select a version")
    //     .default(0)
    //     .items(&index)
    //     .interact()
    //     .unwrap();

    Ok(())
}
