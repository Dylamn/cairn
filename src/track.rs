// TODO: consider a small helper to flatten a Gpx's track(s)/segment(s) into
// a single Vec<Waypoint>, enforcing the v1 single-track/single-segment
// assumption here and returning GpxIoError::UnsupportedTrackStructure if it
// doesn't hold. trim.rs and selection.rs both want "just give me the
// points" and shouldn't each re-implement the walk through
// gpx.tracks[..].segments[..].

use gpx::{Gpx, Waypoint};

#[derive(Debug, thiserror::Error)]
pub enum TrackError {
    // TODO: relevant once flatten_points()/rebuild_from_points() exist —
    // e.g. for the v1 "single track, single segment" assumption mentioned
    // in trim.rs.
    #[error("expected exactly one track with one segment, found {tracks} track(s)")]
    UnsupportedTrackCount { tracks: usize },

    #[error("expected exactly one segment, found {segments} segment(s)")]
    UnsupportedSegmentCount { segments: usize },
}

pub fn flatten_points(gpx: &Gpx) -> Result<&[Waypoint], TrackError> {
    if gpx.tracks.len() != 1 {
        return Err(TrackError::UnsupportedTrackCount {
            tracks: gpx.tracks.len(),
        });
    }

    let track = &gpx.tracks[0];

    if track.segments.len() != 1 {
        return Err(TrackError::UnsupportedSegmentCount {
            segments: gpx.tracks.len(),
        });
    }

    Ok(track.segments[0].points.as_slice())
}
//
// And the inverse, for rebuilding a Gpx from a flat Vec<Waypoint> plus the
// original Gpx (to preserve metadata) — trim_gpx will need this too. Likely
// infallible (building is easier than validating), hence no Result here —
// revisit if that turns out not to be true.
//
pub fn rebuild_from_points(original: &Gpx, points: Vec<Waypoint>) -> Gpx {
    let mut new_gpx = original.clone();
    new_gpx.tracks[0].segments[0].points = points;
    
    new_gpx
}