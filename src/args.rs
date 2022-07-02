use clap::Parser;

#[derive(Parser)]
#[clap(name = "yanvm", about)]
pub struct Args {
    pub command: Option<String>,
}
