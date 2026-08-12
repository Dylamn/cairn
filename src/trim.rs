//! Core trimming logic. Deliberately "dumb" and pure — no file I/O, no CLI
//! concerns. All the interesting "which point did the user mean" work
//! happens in `selection.rs` *before* calling into this module; by the time
//! code here runs, we should already have two concrete indices.
//!
//! Keeping this pure (no I/O, no side effects) makes it trivial to unit
//! test and trivial to reuse verbatim from a future GUI.

use gpx::{Gpx, Waypoint};
use crate::track::{flatten_points, rebuild_from_points};
use crate::TrackError;

// trim.rs's own errors, distinct from SelectionError — this is for
// problems specific to the trim *operation* itself (e.g., a nonsensical
// from/to pair), as opposed to problems resolving a single selector.
#[derive(Debug, thiserror::Error)]
pub enum TrimError {
    #[error("from index ({from}) is after to index ({to})")]
    FromAfterTo { from: usize, to: usize },

    #[error(transparent)]
    Track(#[from] TrackError),
}

// TODO: given a slice of points and a from/to index (inclusive), return the
// trimmed-down Vec<Waypoint>.
//
// Things to decide:
// - from > to: return TrimError::FromAfterTo rather than silently
//   swapping them — swapping silently could surprise a caller who made a
//   genuine mistake (e.g. selectors resolved in an unexpected order).
// - from == to: probably valid (a single-point "track"), worth an
//   explicit test case either way.
// - Out-of-range from/to: this function assumes indices are already
//   validated (by selection::resolve, which returns SelectionError for
//   that). Keep this function's own error surface limited to
//   FromAfterTo — don't duplicate bounds-checking here.
pub fn trim_points(
    points: &[Waypoint],
    from: usize,
    to: usize,
) -> Result<Vec<Waypoint>, TrimError> {
    if from > to {
        return Err(TrimError::FromAfterTo { from, to });
    }
    Ok(points[from..=to].to_vec())
    //todo!("validate from <= to, then slice points[from..=to] and return an owned Vec")
}

// TODO: this is the "whole file" version — takes a full parsed Gpx struct
// (from gpx_io::load) and the two indices, and returns a new Gpx struct
// with the track(s) replaced by the trimmed points.
//
// Things to think about:
// - v1 simplifying assumption: single track, single segment. Document this
//   clearly (maybe even return an error/Result if the input has more than
//   one track/segment, rather than silently doing the wrong thing).
// - What metadata should change? E.g. appending " (trimmed)" to the track
//   or file name, updating <bounds> if the gpx crate doesn't recompute it
//   automatically. Check what the `gpx` crate's writer does with `bounds`
//   -- worth a quick experiment/print-debug rather than assuming.
// - This function should call `trim_points` above rather than duplicating
//   slicing logic — keep the "flatten Gpx -> operate on Vec<Waypoint> ->
//   rebuild Gpx" pattern, since split/merge/simplify will likely follow the
//   same shape later.
//
// Note the `from`/`to` parameters here are already-resolved usize indices,
// not Selectors — resolving Selectors (which can fail with SelectionError)
// is expected to happen one level up, in main.rs, before calling this. That
// keeps this function focused on one thing (assembling a new Gpx) rather
// than also being responsible for selector resolution.
pub fn trim_gpx(gpx: &Gpx, from: usize, to: usize) -> Result<Gpx, TrimError> {
    let waypoints = flatten_points(gpx)?;
    let trimmed_points = trim_points(waypoints, from, to)?;

    let new_gpx = rebuild_from_points(gpx, trimmed_points);

    Ok(new_gpx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_points() {
        todo!("Make a test fixture GPX file, then test trim_points on it")
    }
    // TODO: unit tests for trim_points using small synthetic Vec<Waypoint>
    // fixtures (no need to load real GPX files here). Cases worth covering:
    // - normal trim (from < to, both in range)
    // - from == to (single point result)
    // - from == 0 / to == points.len() - 1 (edges of the track)
    // - maybe a #[should_panic] or Result-based test for out-of-range,
    //   depending on how bounds-checking responsibility gets decided above
}