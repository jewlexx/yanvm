use serde::{Deserialize, Serialize};

pub mod index;

#[derive(Copy, Clone)]
pub enum Arch {
    X86,
    X64,
    ARMv7,
    ARM64,
}

impl Arch {
    pub const fn new() -> Self {
        let arch = if cfg!(target_arch = "x86_64") {
            Arch::X64
        } else if cfg!(target_arch = "x86") {
            Arch::X86
        } else if cfg!(target_arch = "arm") {
            Arch::ARMv7
        } else if cfg!(target_arch = "aarch64") {
            Arch::ARM64
        } else {
            panic!("Unsupported architecture");
        };

        arch
    }
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

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl Version {
    pub const fn new(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl From<(i32, i32, i32)> for Version {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Version> for (i32, i32, i32) {
    fn from(version: Version) -> Self {
        (version.major, version.minor, version.patch)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}
