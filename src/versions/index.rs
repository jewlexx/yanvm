use serde::{Deserialize, Serialize};

use crate::consts::CLIENT;

pub fn parse_version(ver: impl AsRef<str>) -> (i32, i32, i32) {
    let ver_number = ver.as_ref().replace('v', "");
    let mut ver = ver_number.split('.');
    let major = ver.next().unwrap().parse::<i32>().unwrap();
    let minor = ver.next().unwrap().parse::<i32>().unwrap();
    let patch = ver.next().unwrap().parse::<i32>().unwrap();

    (major, minor, patch)
}

fn sort_index(unsorted: &mut NodeIndex) {
    unsorted.sort_by(|ver, old| {
        let ver = parse_version(ver.version.replace('v', ""));
        let ver_cmp = (ver.0 << 16) | (ver.1 << 8) | ver.2;

        let old = parse_version(old.version.replace('v', ""));
        let old_cmp = (old.0 << 16) | (old.1 << 8) | old.2;

        ver_cmp.cmp(&old_cmp).reverse()
    });
}

pub async fn list_index() -> reqwest::Result<NodeIndex> {
    let index: NodeIndex = CLIENT
        .get("https://nodejs.org/dist/index.json")
        .send()
        .await?
        .json()
        .await?;

    let mut filtered: NodeIndex = index
        .iter()
        .filter(|x| x.version.starts_with('v'))
        .map(|x| {
            let mut ver = x.clone();
            ver.version = x.version.replace('\n', "");

            ver
        })
        .collect();

    sort_index(&mut filtered);

    Ok(filtered)
}

pub type NodeIndex = Vec<NodeIndexElement>;

impl std::fmt::Display for NodeIndexElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeIndexElement {
    pub version: String,
    pub date: String,
    pub files: Vec<File>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<Zlib>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    pub lts: LtsUnion,
    pub security: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsUnion {
    Bool(bool),
    Enum(LtsEnum),
}

impl Default for LtsUnion {
    fn default() -> Self {
        LtsUnion::Bool(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum File {
    #[serde(rename = "aix-ppc64")]
    AixPpc64,
    #[serde(rename = "headers")]
    Headers,
    #[serde(rename = "linux-arm64")]
    LinuxArm64,
    #[serde(rename = "linux-armv6l")]
    LinuxArmv6L,
    #[serde(rename = "linux-armv7l")]
    LinuxArmv7L,
    #[serde(rename = "linux-ppc64le")]
    LinuxPpc64Le,
    #[serde(rename = "linux-s390x")]
    LinuxS390X,
    #[serde(rename = "linux-x64")]
    LinuxX64,
    #[serde(rename = "linux-x86")]
    LinuxX86,
    #[serde(rename = "osx-arm64-tar")]
    OsxArm64Tar,
    #[serde(rename = "osx-x64-pkg")]
    OsxX64Pkg,
    #[serde(rename = "osx-x64-tar")]
    OsxX64Tar,
    #[serde(rename = "osx-x86-tar")]
    OsxX86Tar,
    #[serde(rename = "src")]
    Src,
    #[serde(rename = "sunos-x64")]
    SunosX64,
    #[serde(rename = "sunos-x86")]
    SunosX86,
    #[serde(rename = "win-x64-7z")]
    WinX647Z,
    #[serde(rename = "win-x64-exe")]
    WinX64Exe,
    #[serde(rename = "win-x64-msi")]
    WinX64Msi,
    #[serde(rename = "win-x64-zip")]
    WinX64Zip,
    #[serde(rename = "win-x86-7z")]
    WinX867Z,
    #[serde(rename = "win-x86-exe")]
    WinX86Exe,
    #[serde(rename = "win-x86-msi")]
    WinX86Msi,
    #[serde(rename = "win-x86-zip")]
    WinX86Zip,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LtsEnum {
    Argon,
    Boron,
    Carbon,
    Dubnium,
    Erbium,
    Fermium,
    Gallium,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Zlib {
    #[serde(rename = "1.2.11")]
    The1211,
    #[serde(rename = "1.2.3")]
    The123,
    #[serde(rename = "1.2.8")]
    The128,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_index() {
        let index = list_index().await.unwrap();
        assert!(!index.is_empty());
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("v1.2.3"), (1, 2, 3));

        assert_eq!(parse_version("v16.15.1"), (16, 15, 1));
    }

    #[test]
    fn test_sort_index() {
        let mut index = vec![
            NodeIndexElement {
                version: "v1.2.1".to_string(),
                ..Default::default()
            },
            NodeIndexElement {
                version: "v1.2.3".to_string(),
                ..Default::default()
            },
            NodeIndexElement {
                version: "v1.2.2".to_string(),
                ..Default::default()
            },
        ];

        sort_index(&mut index);

        assert_eq!(
            index,
            vec![
                NodeIndexElement {
                    version: "v1.2.3".to_string(),
                    ..Default::default()
                },
                NodeIndexElement {
                    version: "v1.2.2".to_string(),
                    ..Default::default()
                },
                NodeIndexElement {
                    version: "v1.2.1".to_string(),
                    ..Default::default()
                },
            ]
        );
    }
}
