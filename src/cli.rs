//! Command-line argument parsing. Binary-only — NOT part of the `cairn`
//! library (see main.rs, which declares `mod cli;` itself rather than this
//! being declared in lib.rs). Keeps `clap` and CLI-specific concerns out of
//! the library's public API.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

// TODO: top-level CLI struct. `#[command(...)]` attributes let you set the
// binary name/version/about text shown in `--help` — consider pulling
// version from Cargo.toml automatically via `version` (clap supports this
// out of the box).
#[derive(Parser)]
#[command(name = "cairn", about = "A tool for working with GPX files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Trim a GPX track down to the segment between two points.
    Trim {
        /// Path to the input GPX file.
        #[arg(required = true)]
        input: PathBuf,

        // TODO: decide on the actual flag shape. Simplest v1: plain index.
        // --from and --to as strings, parsed later into a
        // cairn::Selector (probably in main.rs, or via a custom clap
        // value_parser here — worth reading up on clap's value_parser
        // feature once this needs to support multiple selector kinds
        // like "0", "12.5km", "48.1,-1.6").
        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,

        /// Path to write the trimmed output GPX file.
        #[arg(short, long)]
        output: PathBuf,
        // TODO: consider a --force flag if gpx_io::save() refuses to
        // overwrite existing files by default (see NOTE in gpx_io.rs).
    },
    // Stats { input: PathBuf },
    // Split { input: PathBuf, at: Vec<String>, output_dir: PathBuf },
    // Merge { inputs: Vec<PathBuf>, output: PathBuf },
}

#[cfg(test)]
mod tests {
    // TODO: clap supports testing argument parsing without running the
    // whole program, e.g.:
    //
    use super::*;
    #[test]
    fn parses_trim_args() {
        let cli = Cli::try_parse_from([
            "cairn", "trim", "in.gpx", "--from", "0", "--to", "10", "--output", "out.gpx",
        ])
        .unwrap();

        match cli.command {
            Command::Trim { input, from, to, output} => {
                assert_eq!(input, PathBuf::from("in.gpx"));
                assert_eq!(from, "0");
                assert_eq!(to, "10");
                assert_eq!(output, PathBuf::from("out.gpx"));
            }
        }
    }
}
