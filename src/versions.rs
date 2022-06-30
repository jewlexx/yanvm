pub enum Arch {
    X86,
    X64,
    ARMv7,
    ARM64,
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Arch::X86 => write!(f, "x86"),
            Arch::X64 => write!(f, "x64"),
            Arch::ARMv7 => write!(f, "armv7l"),
            Arch::ARM64 => write!(f, "arm64"),
        }
    }
}

pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl From<(u32, u32, u32)> for Version {
    fn from(tuple: (u32, u32, u32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Version> for (u32, u32, u32) {
    fn from(version: Version) -> Self {
        (version.major, version.minor, version.patch)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn parse_installer(ver: impl Into<Version>, arch: Arch) -> String {
    let (os, ext) = crate::consts::OS_STR;
    let version: Version = ver.into();

    format!("node-{version}-{os}-{arch}.{ext}")
}
