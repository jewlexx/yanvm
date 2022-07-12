use std::{
    cmp::min,
    fmt::Display,
    fs::create_dir_all,
    io::{Cursor, Read},
    path::PathBuf,
};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    consts::CLIENT,
    helpers::{NoneError, ToError},
    versions::{
        index::{list_index, parse_version, LtsUnion},
        Arch, Version,
    },
};

pub enum ArchiveType {
    TarGz,
    TarXz,
    Zip,
}

impl Display for ArchiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_ext())
    }
}

impl ArchiveType {
    pub fn get_ext(&self) -> &'static str {
        match self {
            ArchiveType::TarGz => "tar.gz",
            ArchiveType::TarXz => "tar.xz",
            ArchiveType::Zip => "zip",
        }
    }
}

pub struct Archive {
    pub dirs: Vec<PathBuf>,
    pub files: Vec<(PathBuf, Vec<u8>)>,
}

impl Archive {
    pub fn total_len(&self) -> usize {
        self.files.len() + self.dirs.len()
    }

    pub fn decompress(self) -> std::io::Result<()> {
        let total = self.total_len();

        // Indicatif setup
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
                .progress_chars("#>-"),
        );
        pb.set_message("Unzipping");

        let mut extracted = 0;

        for dir in self.dirs {
            pb.set_message(format!("Creating {}", dir.display()));

            let new = min(extracted + 1, total);
            extracted = new;

            create_dir_all(dir)?;
            pb.set_position(new as u64);
        }

        for file in self.files {
            pb.set_message(format!("Unzipping {}", file.0.display()));

            let new = min(extracted + 1, total);
            extracted = new;

            std::fs::write(file.0, file.1)?;
            pb.set_position(new as u64);
        }

        Ok(())
    }
}

pub struct Decompressor {
    bytes: Cursor<Vec<u8>>,
}

impl Decompressor {
    pub const fn new(bytes: Cursor<Vec<u8>>) -> Self {
        Self { bytes }
    }

    pub fn decompress_into_mem(self, path: PathBuf) -> std::io::Result<Archive> {
        let mut final_archive = Archive {
            dirs: Vec::new(),
            files: Vec::new(),
        };

        // TODO: fix issues with cross platform decompression
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                let mut unzipped = zip::read::ZipArchive::new(self.bytes)?;

                let total = unzipped.len();

                // // Indicatif setup
                // let pb = ProgressBar::new(total as u64);
                // pb.set_style(
                //     ProgressStyle::default_bar()
                //         .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
                //         .progress_chars("#>-"),
                // );
                // pb.set_message("Unzipping");

                let mut downloaded = 0;

                for i in 0..unzipped.len() {
                    let mut file = unzipped.by_index(i)?;

                    let path = path.join(file.enclosed_name().unwrap());
                    if file.is_dir() {
                        final_archive.dirs.push(path);
                    } else if file.is_file() {
                        let mut unpacked: Vec<u8> = Vec::new();

                        file.read_to_end(&mut unpacked);

                        final_archive.files.push((path, unpacked));
                    }
                }

                pb.finish_with_message("Unzipped");
            } else {
                cfg_if::cfg_if! {
                    if #[cfg(target_os = "macos")] {
                        let unzipped = flate2::read::GzDecoder::new(self.bytes);
                    } else {
                        let unzipped = xz2::read::XzDecoder::new(self.bytes);
                    }
                }

                let mut archive = tar::Archive::new(unzipped);

                let entries = archive.entries()?;

                for entry in entries {
                    let mut entry = entry.unwrap();

                    let path = path.join(entry.path()?);

                    match (*entry.header()).entry_type() {
                        tar::EntryType::Directory => final_archive.dirs.push(path),
                        tar::EntryType::Regular => {
                            let mut unpacked: Vec<u8> = Vec::new();

                            entry.read_to_end(&mut unpacked)?;

                            final_archive.files.push((path, unpacked));
                        },
                        // TODO: Figure out exactly what types of files will exist in the archive and handle all possibilities.
                        _ => todo!(),
                    }
                }
            }
        }
        Ok(final_archive)
    }
}

pub struct NodeBinary {
    bytes: Cursor<Vec<u8>>,
}

impl NodeBinary {
    pub fn new(bytes: Vec<u8>) -> Self {
        let cursor = Cursor::new(bytes);

        Self { bytes: cursor }
    }

    pub async fn unzip_file(self) -> Result<(), InstallError> {
        // NOTE: This is temporary until I figure out actual dirs
        let path = std::env::current_dir().unwrap();

        let archive = Decompressor::new(self.bytes).decompress_into_mem(path)?;

        archive.decompress()?;

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
    #[cfg(windows)]
    #[error("Failed to decompress file: {0}")]
    DecompressError(#[from] zip::ZipError),
    #[error("Interaction with config")]
    ConfigError(#[from] crate::config::ConfigError),
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

        let mut config = crate::consts::CONFIG.lock();

        config.versions.push(self.version);
        config.save()?;

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
