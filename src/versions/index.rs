use reqwest::Client;
use serde::{Deserialize, Serialize};

fn parse_version(ver: String) -> u32 {
    let mut ver = ver.split('.');
    let major = ver.next().unwrap().parse::<u32>().unwrap();
    let minor = ver.next().unwrap().parse::<u32>().unwrap();
    let patch = ver.next().unwrap().parse::<u32>().unwrap();

    (major << 16) | (minor << 8) | patch
}

pub async fn list_index(client: &Client) -> reqwest::Result<NodeIndex> {
    let index: NodeIndex = client
        .get("https://nodejs.org/dist/index.json")
        .send()
        .await?
        .json()
        .await?;

    let mut filtered: NodeIndex = index
        .iter()
        .filter(|x| x.version.starts_with('v'))
        .cloned()
        .collect();

    filtered.sort_by(|ver, old| {
        let ver = parse_version(ver.version.replace('v', ""));
        let old = parse_version(old.version.replace('v', ""));

        ver.cmp(&old)
    });

    Ok(filtered)
}

pub type NodeIndex = Vec<NodeIndexElement>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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