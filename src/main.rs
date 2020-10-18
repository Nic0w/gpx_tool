extern crate serde;
extern crate quick_xml;

use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use simple_xml_serialize::XMLElement;

use quick_xml::de::{from_str, DeError};

mod gpx;
mod distance;
mod tsp;

use crate::gpx::*;
use crate::distance::distance;
use crate::tsp::solve_tsp;

fn main() {
    
    println!("Hello, world!");

    let path = Path::new("avenue-verte-london-paris-maisons-laffitte-chaussy.gpx");
    println!("Opening file '{}'", path.display());

    let mut gpx_file = match File::open(&path) {    
        Err(reason) => panic!("Failed to open file: {}", reason),
        Ok(file)    => file
    };

    let mut content = String::new();

    match gpx_file.read_to_string(&mut content) {
        
        Err(reason) => panic!("Failed to read file: {}", reason),
        Ok(size) => println!("Read {} bytes !", size)
    };

    let gpx: Gpx = match from_str(&content) {
    
        Err(reason) => panic!("Failed to parse data: {}", reason),
        Ok(data) => data
    };

    println!("File by '{}', version {}", gpx.creator, gpx.version);
    println!("Parsed {} tracks !", gpx.tracks.len());

    let mut unique_points: HashSet<TrackPoint> = HashSet::new();
    let mut final_points: Vec<(f64, f64)> = Vec::new();
    let mut point_count = 0usize;
    let mut previous = (std::f64::NAN, std::f64::NAN);
    let mut avg_dist: f64 = 50.0; 
    let mut outlier: bool = false;
    for (i, track) in gpx.tracks.iter().enumerate() {
        //println!("{} segments in track {}.", track.segments.len(), i);
        
        for segment in track.segments.iter() { 
            //println!("\t{} points in segment.", segment.points.len());

            for point in segment.points.iter() {
           
                /*let current = (point.lat, point.lon);
                let dist = distance(&previous, &current);
                previous = current;*/

                if unique_points.insert(point.clone()) {
                    final_points.push((point.lat, point.lon));
                }
            }
        }

        point_count += track.segments[0].points.len()
    }

    println!("{} points in total, kept {}.", point_count, final_points.len());

    let ordered = solve_tsp(final_points);

    println!("We still have {} points after TSP solver.", ordered.len());

    let mut final_ordered: Vec<TrackPoint> = Vec::new();

    for point in ordered.iter() {
        
        final_ordered.push(TrackPoint {
            lat: point.0,
            lon: point.1,

            elevations: vec![]
        });
    }

    let cleanedup_gpx = Gpx {
        creator: "nic0w".to_string(),
        version: "1.1".to_string(),

        metadata: Metadata {
            name: Name { value: "Test".to_string() }
        },

        tracks: vec![Track {
            name: Name { value: "TestSeg".to_string() },
            segments: vec![TrackSegment{
                points: final_ordered.clone()
            }]
        }]
    };

    let cleanedup_xml = XMLElement::from(cleanedup_gpx);

    println!("{}", cleanedup_xml.to_string_pretty("\n", "  "));
}
