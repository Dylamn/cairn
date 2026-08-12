//! Resolving a "human" way of pointing at a spot in a track (an index, a
//! timestamp, a distance along the track, a coordinate...) into a concrete
//! point index that `trim.rs` (and later other operations) can use.
//!
//! This module is deliberately CLI-agnostic and GUI-agnostic: it just knows
//! how to go from "a Selector" to "an index into a slice of points". The CLI
//! is responsible for turning raw strings (like "12.5km" or "0") into a
//! `Selector`. A future GUI would build `Selector` values directly (e.g.
//! `Selector::Coordinate` from a map click) without ever touching strings.

use gpx::Waypoint;
use time::OffsetDateTime;

// Precise, typed errors for this module. `thiserror` generates the
// `Display` impl from the `#[error("...")]` strings below, and (since the
// enum implements std::error::Error) these convert automatically into
// anyhow::Error via `?` in main.rs, and into cairn::Error via `#[from]`
// once that's wired up in lib.rs.
#[derive(Debug, thiserror::Error)]
pub enum SelectionError {
    #[error("index {index} out of range (track has {len} points)")]
    IndexOutOfRange { index: usize, len: usize },

    #[error("track has no timestamps, cannot select a point by time")]
    NoTimestamps,

    #[error("track has no points, cannot resolve a selector")]
    EmptyTrack,

    #[error("requested time is outside the track's recorded range")]
    TimeOutOfRange,
}

// TODO: derive whatever traits make sense (Debug, Clone, PartialEq at least
// — useful for tests and for clap's value_parser error messages).
//
// Each variant represents a different way of "pointing" at a spot in the
// track. Only `Index` needs to be implemented for the very first version;
// the others are here so the shape of the type doesn't need to change later.
pub enum Selector {
    /// Raw index into the flattened list of points. Simplest, least
    /// user-friendly. Good for v1 / for scripting.
    Index(usize),

    /// A timestamp. Requires points to actually have `time` set — decide
    /// how to handle points/tracks that don't (error? skip?).
    Time(OffsetDateTime), // or chrono::DateTime<Utc> — pick one time crate and stick to it

    /// Distance along the track from the start, in kilometers (or meters —
    /// decide on a unit and be consistent everywhere, including in the CLI
    /// flag name, e.g. --from-km vs. --from-m).
    DistanceKm(f64),

    /// Nearest point to a given (lat, lon). This is the one the future GUI
    /// will use the most (map click -> nearest point on track).
    Coordinate { lat: f64, lon: f64 },
}

// TODO: given a Selector and the full ordered list of points, return the
// resolved index into that list.
//
// Notes / things to figure out per variant:
// - Index: bounds-check against points.len(); return
//   SelectionError::IndexOutOfRange rather than panicking.
// - DistanceKm: needs cumulative distance along the track first (see
//   `cumulative_distances_km` below) then find the closest match.
// - Coordinate: needs point-to-point distance (haversine) to find the
//   single nearest point; consider what "nearest" means if the track
//   crosses itself (loop hikes) — probably fine to just take the first
//   closest match for v1.
//
// Should probably check `points.is_empty()` first thing and return
// SelectionError::EmptyTrack, since every variant below needs at least one
// point to make sense of.
pub fn resolve(selector: &Selector, points: &[Waypoint]) -> Result<usize, SelectionError> {
    if points.is_empty() {
        return Err(SelectionError::EmptyTrack);
    }

    match selector {
        Selector::Index(index) => {
            if *index >= points.len() {
                return Err(SelectionError::IndexOutOfRange {
                    index: *index,
                    len: points.len(),
                });

            }

            Ok(*index)
        },
        _ => panic!("Unimplemented selector variant. Only Index is supported for now."),
    }
}

// TODO: compute the cumulative distance (in km) from the start of the track
// to each point. Returns a Vec<f64> the same length as `points`, where
// result[i] = distance from points[0] to points[i] along the track.
//
// Needed by: DistanceKm selector resolution, and later by the `stats`
// command (total distance) and elevation-profile export.
//
// Look at geo/geo-types for a haversine (or Vincenty, if more precision is
// ever needed) distance function between two points rather than
// hand-rolling the formula.
pub fn cumulative_distances_km(points: &[Waypoint]) -> Vec<f64> {
    todo!("compute cumulative distance along the track")
}

// TODO: given a target lat/lon, find the index of the nearest point in
// `points`. Used by Selector::Coordinate resolution today, and directly
// reusable later for a GUI's "click on map -> find nearest point" feature —
// this is the "bridge function" mentioned in earlier planning.
//
// Only real failure case is an empty track (EmptyTrack) — there's always a
// "nearest" point otherwise, however far away it is.
pub fn nearest_point_index(
    points: &[Waypoint],
    lat: f64,
    lon: f64,
) -> Result<usize, SelectionError> {
    todo!("find index of point nearest to (lat, lon)")
}

#[cfg(test)]
mod tests {
    // TODO: unit tests once resolve()/cumulative_distances_km() have real
    // implementations. Build small synthetic Vec<Waypoint> fixtures here
    // rather than loading real GPX files — keeps these tests fast and
    // independent of gpx_io.rs.
}