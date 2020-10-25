use std::path::Path;

use crate::gpx::{Track, TrackPoint, parse_gpx, to_gpx};


macro_rules! get_point {
    ($src:ident, $func:ident, $var:ident) => {

        let segment = $src.segments.$func();
        if let Some(segment) = segment {
            let point = segment.points.$func();
            if let Some(point) = point {
                $var = (point.lat, point.lon);
            }
        }
    };
}

pub fn info(file: Option<&str>){

    let path = Path::new(file.unwrap());

    let gpx_data = parse_gpx(&path);

    println!("File is titled '{}', made by '{}' (version '{}').", gpx_data.metadata.name.value, gpx_data.creator, gpx_data.version);

    println!("-> contains {} tracks.", gpx_data.tracks.len());

    let mut total_points: usize = 0;
    let mut first_point: (f64, f64) = (f64::NAN, f64::NAN);
    let mut last_point: (f64, f64)= (f64::NAN, f64::NAN);

    for (i, track) in gpx_data.tracks.iter().enumerate() {

        if i == 0 {

            get_point!(track, first, first_point);
        }

        if i+1 == gpx_data.tracks.len() {

            get_point!(track, last, last_point);
        }

        let point_count = track.segments.iter().
            fold(0, |acc, seg| acc + seg.points.len());
        total_points += point_count;
        println!("\t-> track {} ('{}') contains {} segments, totalling {} points.", i, track.name.value, track.segments.len(), point_count);
    }

    println!("-> {} points in total.", total_points);
    println!("-> first point: ({}, {})", first_point.0, first_point.1);
    println!("-> last point: ({}, {})", last_point.0, last_point.1);
}
