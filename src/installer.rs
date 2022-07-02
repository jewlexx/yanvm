use std::{cmp::min, fs::File, io::Write};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use crate::{
    helpers::ToError,
    versions::{
        index::{list_index, parse_version, LtsUnion},
        Arch, Version,
    },
};

pub struct Installer {
    version: Version,
    arch: Arch,
}

impl Installer {
    pub const fn new(version: Version, arch: Arch) -> Self {
        Self { version, arch }
    }

    pub fn get_installer_link(&self) -> String {
        let installer = self.parse_installer();

        format!("https://nodejs.org/dist/{}/{installer}", self.version)
    }

    pub fn parse_installer(&self) -> String {
        let (os, ext) = crate::consts::OS_STR;

        format!("node-{}-{os}-{}.{ext}", self.version, self.arch)
    }

    pub async fn lts_version() -> anyhow::Result<Self> {
        let index = list_index().await?;
        let version_string = index
            .iter()
            .find(|i| i.lts != LtsUnion::Bool(false))
            .to_error()?
            .version
            .to_string();

        let version: Version = parse_version(version_string).into();
        let installer = Installer::new(version, Arch::new());

        Ok(installer)
    }

    pub async fn latest_version() -> anyhow::Result<Self> {
        let index = list_index().await?;

        let version: Version = parse_version(&index[0].version).into();

        let installer = Installer::new(version, Arch::new());

        Ok(installer)
    }

    pub async fn download_binary(&self, client: &Client) -> anyhow::Result<()> {
        let link = self.get_installer_link();

        let res = client.get(link.clone()).send().await?;

        let total_size = res.content_length().unwrap_or(0);

        // Indicatif setup
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", link));

        let path = self.parse_installer();

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
}

#[cfg(test)]
mod tests {
    use super::*;

    static VERSION: Version = Version::new(1, 2, 3);

    #[test]
    fn test_version() {
        assert_eq!(VERSION.major, 1);
        assert_eq!(VERSION.minor, 2);
        assert_eq!(VERSION.patch, 3);

        assert_eq!(VERSION.to_string(), "v1.2.3");
    }

    #[test]
    fn test_installer() {
        let installer = Installer::new(VERSION, Arch::new());

        let (os, ext) = crate::consts::OS_STR;

        assert_eq!(
            installer.get_installer_link(),
            format!("node-v1.2.3-{os}-x64.{ext}")
        );
    }
}
