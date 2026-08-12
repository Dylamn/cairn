//! Loading and saving GPX files. All the "touching the filesystem / parsing
//! XML" concerns live here, kept separate from trim.rs's pure logic and
//! from selection.rs's point-resolution logic.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use gpx::Gpx;

// Precise, typed errors for this module's file/parse concerns. Using
// #[source] on the underlying io::Error / parse error lets anyhow (in
// main.rs) print the full "caused by" chain, rather than just this
// wrapper's own message.
#[derive(Debug, thiserror::Error)]
pub enum GpxIoError {
    #[error("failed to open file at {path}")]
    Open {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },

    // #[from] auto-generates `impl From<gpx::errors::GpxError> for
    // GpxIoError`, so `?` converts automatically inside load().
    #[error("failed to parse GPX content")]
    Parse(#[from] gpx::errors::GpxError),

    #[error("failed to write file at {path}")]
    Write {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("output file already exists: {0} (refusing to overwrite)")]
    OutputExists(std::path::PathBuf),
}

// TODO: open the file at `path`, parse it with the `gpx` crate, return the
// parsed Gpx struct.
//
// Things to think about:
// - Wrap the File::open() error into GpxIoError::Open (with `path` and
//   `source`) rather than letting a bare io::Error escape — that's what
//   gives the CLI/GUI a message like "failed to open file at ..." instead
//   of a generic OS error.
// - Once GpxIoError::Parse(#[from] ...) is uncommented above, the gpx
//   crate's parse error can just be `?`'d directly inside this function.
// - The gpx crate's read() function likely wants something implementing
//   `std::io::Read` — BufReader<File> is the standard choice for parsing
//   performance on larger files.
// - Consider whether to validate here that the file has exactly one
//   track/segment (matching the v1 simplifying assumption from trim.rs),
//   or leave that check to the caller. Leaning towards: leave it to the
//   caller (trim_gpx), since load() should just be "load whatever is
//   there" and stay reusable for future commands that don't have the same
//   restriction (e.g. stats could handle multi-track files fine).

pub fn load(path: &Path) -> Result<Gpx, GpxIoError> {
    let file = File::open(path).map_err(|e| GpxIoError::Open {
        path: path.to_owned(),
        source: e,
    })?;
    let reader = BufReader::new(file);
    let gpx = gpx::read(reader)?;

    Ok(gpx)
}

// TODO: serialize `gpx` and write it to `path`.
//
// Things to think about:
// - "Non-destructive" is a core promise of this tool — see the
//   OutputExists check noted above the signature.
// - Check what gpx::write() expects (likely a `std::io::Write` sink) and
//   whether it needs a `&Gpx` or an owned `Gpx`.
// - Wrap File::create() failures into GpxIoError::Write, same pattern as
//   Open above — including the case where parent directories don't exist,
//   so the user gets a clear message rather than a raw io::Error.
// Should check path.exists() first and return GpxIoError::OutputExists
// before attempting to write, per the "non-destructive by default"
// decision noted below — unless/until a --force flag is added in cli.rs
// to explicitly allow overwriting.

pub fn save(gpx: &Gpx, path: &Path) -> Result<(), GpxIoError> {
    if path.exists() {
        return Err(GpxIoError::OutputExists(path.to_owned()));
    }

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(GpxIoError::Write {
                path: path.to_owned(),
                source: e,
            });
        }
    };

    gpx::write(gpx, file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: integration-style tests using a small fixture .gpx file under
    // tests/fixtures/ (or src/gpx_io.rs-local test data) — load a known
    // file, assert point count / structure, maybe a round-trip test
    // (load -> save -> load again -> compare).
}
