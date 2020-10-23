use std::path::Path;

use crate::gpx::{Track, TrackPoint, parse_gpx, to_gpx, trackpoint2point};

pub fn combine(paths: Vec<&str>) {

    let mut all_trackpoints: Vec<TrackPoint> = Vec::new();

    for path in paths.iter().map(Path::new) {

        let gpx_data = parse_gpx(path);

        for tracks in gpx_data.tracks.iter() {

            for segments in tracks.segments.iter() {

                all_trackpoints.append(&mut segments.points);
            }
        }
    }

    to_gpx(
        &all_trackpoints.iter().
            map(trackpoint2point).
            collect(),
        None,
        None,
        None,
        None
    );
}
