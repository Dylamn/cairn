use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cairn", about = "A tool for working with GPX files")]
pub struct Cli {
    /// Path to the GPX file
    #[arg(required = true)]
    pub path: PathBuf,
}
