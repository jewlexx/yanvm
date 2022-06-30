use std::{cmp::min, fs::File, io::Write, path::PathBuf};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use crate::versions::{installer_link, parse_installer, Arch, Version};

pub async fn download_binary(
    client: &Client,
    ver: impl Into<Version>,
    arch: Arch,
) -> anyhow::Result<()> {
    let version: Version = ver.into();

    let link = installer_link(version, arch);

    let res = client.get(link.clone()).send().await?;

    let total_size = res.content_length().unwrap_or(0);

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", link));

    let path = parse_installer(version, arch);

    // download chunks
    let mut file = File::create(path.clone())?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", link, path));
    Ok(())
}
