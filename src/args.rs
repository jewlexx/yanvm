use clap::Parser;

#[derive(Parser)]
#[clap(name = "yanvm", about)]
pub struct Args {
    bar: String,
}
