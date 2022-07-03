use std::{
    cmp::min,
    fs::File,
    io::{Cursor, Write},
    path::PathBuf,
};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use zip::result::ZipError;

use crate::{
    consts::CLIENT,
    helpers::{NoneError, ToError},
    versions::{
        index::{list_index, parse_version, LtsUnion},
        Arch, Version,
    },
};

pub struct NodeBinary {
    bytes: Cursor<Vec<u8>>,
}

impl NodeBinary {
    pub fn new(bytes: Vec<u8>) -> Self {
        let cursor = Cursor::new(bytes);

        Self { bytes: cursor }
    }

    pub fn bytes(&self) -> &Cursor<Vec<u8>> {
        &self.bytes
    }

    pub async fn unzip_file(self) -> Result<(), InstallError> {
        let mut unzipped = zip::read::ZipArchive::new(self.bytes)?;

        let total = unzipped.len();

        // Indicatif setup
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
                .progress_chars("#>-"),
        );
        pb.set_message("Unzipping");

        let mut downloaded = 0;

        for i in 0..unzipped.len() {
            let file = unzipped.by_index(i)?;

            let new = min(downloaded + 1, total);
            downloaded = new;

            pb.set_message(format!("Unzipping {}", file.name()));

            pb.set_position(new as u64);
        }

        pb.finish_with_message("Unzipped");

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("{0}")]
    NoneError(#[from] NoneError),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to interact with IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to unzip file: {0}")]
    UnzipError(#[from] ZipError),
}

pub struct Installer {
    version: Version,
    arch: Arch,
}

impl Installer {
    pub const fn new(version: Version, arch: Arch) -> Self {
        Self { version, arch }
    }

    pub fn get_installer_link(&self) -> String {
        format!(
            "https://nodejs.org/dist/{}/{installer}",
            self.version,
            installer = self.parse_installer()
        )
    }

    pub fn parse_installer(&self) -> String {
        let (os, ext) = crate::consts::OS_STR;

        format!(
            "node-{}-{os}-{}.{ext}",
            self.version,
            self.arch,
            os = os,
            ext = ext
        )
    }

    pub async fn lts_version() -> Result<Self, InstallError> {
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

    pub async fn latest_version() -> Result<Self, InstallError> {
        let index = list_index().await?;

        let version: Version = parse_version(&index[0].version).into();

        let installer = Installer::new(version, Arch::new());

        Ok(installer)
    }

    pub async fn download_binary(&self, base_path: PathBuf) -> Result<NodeBinary, InstallError> {
        let link = self.get_installer_link();

        let res = CLIENT.get(link.clone()).send().await?;

        let total_size = res.content_length().unwrap_or(0);

        // Indicatif setup
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", self.version));

        let path = base_path.join(self.parse_installer());

        // download chunks
        // let mut file = File::create(path.clone())?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        let mut bytes: Vec<u8> = Vec::new();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            let iter = chunk.iter();
            bytes.append(&mut iter.copied().collect::<Vec<u8>>());
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!("Downloaded {} to {}", link, path.display()));

        Ok(NodeBinary::new(bytes))
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
            format!("node-v1.2.3-{os}-x64.{ext}", os = os, ext = ext)
        );
    }
}
