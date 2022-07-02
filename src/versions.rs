pub mod index;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Version {
    fn new(major: i32, minor: i32, patch: i32) -> Self {
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

pub fn parse_installer(ver: impl Into<Version>, arch: Arch) -> String {
    let (os, ext) = crate::consts::OS_STR;
    let version: Version = ver.into();

    format!("node-{version}-{os}-{arch}.{ext}")
}

pub fn installer_link(ver: impl Into<Version>, arch: Arch) -> String {
    let version: Version = ver.into();
    let installer = parse_installer(version, arch);

    format!("https://nodejs.org/dist/{version}/{installer}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = super::Version::new(1, 2, 3);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);

        assert_eq!(version.to_string(), "v1.2.3");

        let installer = parse_installer(version, Arch::X64);

        let (os, ext) = crate::consts::OS_STR;

        assert_eq!(installer, format!("node-v1.2.3-{os}-x64.{ext}"));
    }
}
