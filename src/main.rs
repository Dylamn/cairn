//! Binary entry point. Should stay thin: parse args, dispatch to the
//! `cairn` library, handle/report errors. No GPX/trimming logic here —
//! that all lives in the library (see lib.rs) so it can be reused by a
//! future GUI.

mod cli;

use clap::Parser;
use cairn;
use cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Trim { input, from, to, output } => {
            // TODO: rough flow to implement:
            // 1. cairn::load(&input)?  -- parse the input GPX file
            let gpx_orig = cairn::load(&input)?;
            println!("loaded {:?} points", gpx_orig.tracks[0].segments[0].points.len());

            // 2. parse `from`/`to` strings into cairn::Selector values.
            //    For v1 (index-only), this is just from.parse::<usize>()
            //    wrapped in Selector::Index. Once selection.rs supports
            //    more selector kinds, this is where the string gets
            //    sniffed/dispatched (e.g., does it look like "12.5km"? a
            //    "lat,lon" pair? otherwise assume a plain index).
            let from_selector = cairn::Selector::Index(from.parse::<usize>()?);
            let to_selector = cairn::Selector::Index(to.parse::<usize>()?);
            
            // Flatten
            let waypoints = cairn::flatten_points(&gpx_orig)?;
            let from_index = cairn::resolve(&from_selector, waypoints)?;
            let to_index = cairn::resolve(&to_selector, waypoints)?;

            // 4. cairn::trim_gpx(&gpx, from_index, to_index) -- or
            //    trim_points if working directly with the flattened Vec.
            let trimmed = cairn::trim_gpx(&gpx_orig, from_index, to_index)?;

            // 5. cairn::save(&trimmed, &output)?
            cairn::save(&trimmed, &output)?;
            // 6. Print a friendly confirmation message (e.g., point count
            //    before/after, output path).
            println!("Trim success.");
            println!("Saved trimmed GPX to {:?}", output);
            // Error handling: anyhow::Result + `?` throughout should be
            // enough for the binary; the library's own errors (once
            // selection.rs has SelectionError, etc.) will convert into
            // anyhow::Error automatically via the `?` operator as long as
            // they implement std::error::Error (which #[derive(thiserror::Error)]
            // gives you for free).
            Ok(())
        }
    }
}