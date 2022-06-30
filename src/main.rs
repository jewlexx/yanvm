use futures_util::FutureExt;
use reqwest::Client;

use crate::{install::download_binary, versions::installer_link};

mod consts;
mod install;
mod versions;

fn main() {
    let client = Client::new();

    let version = (16, 15, 1);

    let fut = download_binary(&client, version, versions::Arch::X64);

    fut.poll_unpin(cx);
}
