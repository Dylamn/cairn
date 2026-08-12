//! Cairn's core library: platform-agnostic GPX manipulation logic.
//!
//! Deliberately has NO knowledge of the CLI (see cli.rs / main.rs, which
//! live outside this crate's module tree) so the same logic can later be
//! reused by a GUI without modification.

mod gpx_io;
mod selection;
mod track;
mod trim;

// TODO: re-export the functions/types main.rs (and later a GUI crate) will
// actually need to call. Keep this list intentional — it's the crate's
// public API surface. Everything not re-exported here stays an internal
// implementation detail of the library.
//
// Likely candidates based on what's sketched in each module so far:
pub use trim::{trim_points, trim_gpx, TrimError};
pub use selection::{Selector, resolve, SelectionError};
pub use gpx_io::{load, save, GpxIoError};
pub use track::{flatten_points, TrackError};

// A single top-level error type wrapping each module's own error enum.
// #[error(transparent)] forwards Display/source straight through to the
// inner error, so messages don't get a confusing extra wrapper layer, e.g.
// printing a GpxIoError::Open through cairn::Error still just reads
// "failed to open file at ...".
//
// Public functions in this crate should generally return `cairn::Result<T>`
// (defined below) rather than a specific module's error type directly, so
// callers (CLI, future GUI) have one discoverable error type for the whole
// public API, while still being able to `match` on the inner variant for
// precise handling if they want to.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Selection(#[from] SelectionError),

    #[error(transparent)]
    GpxIo(#[from] GpxIoError),

    #[error(transparent)]
    Trim(#[from] TrimError),

    #[error(transparent)]
    Track(#[from] TrackError),
}

pub type Result<T> = std::result::Result<T, Error>;

// NOTE: TrimError itself already wraps SelectionError (see trim.rs) since
// trim_gpx will need to resolve selectors internally in some designs —
// revisit whether that's still the right shape once main.rs's actual flow
// (load -> resolve -> trim -> save) is implemented; there may be a cleaner
// split where trim.rs never touches SelectionError at all and main.rs
// does all the resolving before calling trim_points/trim_gpx.
