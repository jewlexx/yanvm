use crate::versions::installer_link;

mod consts;
mod versions;

fn main() {
    let version = (16, 15, 1);

    let install = installer_link(version, versions::Arch::X64);

    println!("{}", install);
}
