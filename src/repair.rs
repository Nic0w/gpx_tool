use std::path::Path;
use std::collections::HashSet;

use crate::gpx::{Track, TrackPoint, parse_gpx};
use crate::lookup::{directional_lookup, coordinates_lookup, CardinalPoint};
use crate::tsp::solve_tsp;
use crate::distance::distance;

pub fn repair(file: Option<&str>, cardinal_point: Option<&str>, start_point: Option<&str>) {

    let path = Path::new(file.unwrap());

    println!("Opening file '{}'", path.display());

    let gpx_data = parse_gpx(&path);

    println!("File by '{}', version {}", gpx_data.creator, gpx_data.version);
    println!("Parsed {} tracks !", gpx_data.tracks.len());

    let (total_points, unique_points) = dedup_trackpoints(&gpx_data.tracks);

    println!("{} points in total, kept {}.", total_points, unique_points.len());

    let startpoint_index = select_startpoint(cardinal_point, start_point, &unique_points);

    let start_point = unique_points[startpoint_index];

    println!("Starting with ({}, {})", start_point.0, start_point.1);

    let mut ordered_points = solve_tsp(startpoint_index, &unique_points);

    let mut previous = ordered_points[0];
    let mut average = 0f64;
    let mut prev_avg = average;
    for p in ordered_points.iter() {

        let d = distance(&previous, p);
        prev_avg = average;
        average = (d + average)/2.0;
        println!("distance from previous: {}; average: {} {} {}", d, average, d.sqrt(), d.sqrt()>prev_avg);

        previous = *p;
    }

}

fn select_startpoint(cardinal_point: Option<&str>, start_point: Option<&str>, points: &Vec<(f64, f64)>) -> usize {
    match cardinal_point {
        Some(cardinal_point_str) => {
            let cardinal_point: CardinalPoint = cardinal_point_str.parse().unwrap();

            directional_lookup(cardinal_point, points)
        },

        _ => match start_point {
            Some(point_str) => {

                let coordinates: Vec<&str> = point_str.split(",").collect();

                let point = (coordinates[0].parse().unwrap(), coordinates[1].parse().unwrap());

                coordinates_lookup(point, points)
            },

            _ => { println!("Defaulting to first point as start point!"); 0usize }
        }
    }
}

fn dedup_trackpoints(tracks: &Vec<Track>) -> (usize, Vec<(f64, f64)>) {

    let mut unique_points: HashSet<TrackPoint> = HashSet::new();
    let mut final_points: Vec<(f64, f64)> = Vec::new();
    let mut point_count = 0usize;

    for track in tracks.iter() {

        for segment in track.segments.iter() {

            for point in segment.points.iter() {

                if unique_points.insert(point.clone()) {
                    final_points.push((point.lat, point.lon));
                }
            }
        }

        point_count += track.segments[0].points.len()
    }

    (point_count, final_points)
}
