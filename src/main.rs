mod cli;

use clap::{Parser};
use cli::Cli;

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
    println!("{:?}", args.path.display())
}
