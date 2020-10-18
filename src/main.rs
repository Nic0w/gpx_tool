extern crate serde;
extern crate quick_xml;
extern crate clap;

use std::env;

use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use simple_xml_serialize::XMLElement;

use quick_xml::de::{from_str, DeError};

use clap::{App, Arg, SubCommand};

mod gpx;
mod distance;
mod tsp;
mod lookup;

use crate::gpx::*;
use crate::distance::distance;
use crate::tsp::solve_tsp;
use crate::lookup::{directional_lookup, coordinates_lookup, CardinalPoint};

struct ParsedArgs {
    file: String,
    start: (f64, f64)
}

fn main() {
    
    let args = App::new("GPX Parser").
        version("1.0").
        author("nic0w").
        about("Fixes broken gpx files").
        arg(Arg::with_name("file").
            short("f").
            long("file").
            value_name("FILE").
            help("File to fix").
            takes_value(true).
            required(true)).
        arg(Arg::with_name("start-point").
            short("s").
            long("start").
            value_name("lat,lon").
            help("When reconstructing a broken track, this is where the track will begin").
            takes_value(true)).
        arg(Arg::with_name("cardinal-point").
            short("c").
            long("cardinal").
            value_name("North|South|East|West").
            help("When reconstructing a broken track, the algorithm will look for the easternmost, westernmost, southernmost, northenmost point to use as a starting point").
            takes_value(true)).
        get_matches();

    println!("Hello, world!");  

    let path = Path::new(args.value_of("file").unwrap());
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
    for (i, track) in gpx.tracks.iter().enumerate() {
        
        for segment in track.segments.iter() { 

            for point in segment.points.iter() {
           
                if unique_points.insert(point.clone()) {
                    final_points.push((point.lat, point.lon));
                }
            }
        }

        point_count += track.segments[0].points.len()
    }

    println!("{} points in total, kept {}.", point_count, final_points.len());

    //(48.947646f64, 2.153013f64)


    let startpoint_index = match args.value_of("cardinal-point") {
        Some(cardinal_point_str) => {
            let cardinal_point: CardinalPoint = cardinal_point_str.parse().unwrap();

            directional_lookup(cardinal_point, &final_points)
        },

        _ => match args.value_of("start-point") {
            Some(point_str) => {
            
                let coordinates: Vec<&str> = point_str.split(",").collect();
                
                let point = (coordinates[0].parse().unwrap(), coordinates[1].parse().unwrap());

                coordinates_lookup(point, &final_points)
            },
            
            _ => { println!("Defaulting to first point as start point!"); 0usize }
        }
    };

    let start_point = final_points[startpoint_index];

    println!("Starting with ({}, {})", start_point.0, start_point.1);

    let mut ordered = solve_tsp(startpoint_index, final_points);

    println!("We still have {} points after TSP solver.", ordered.len());

    let mut final_ordered: Vec<TrackPoint> = Vec::new();

    println!("{} {}", ordered[ordered.len()-1].0, ordered[ordered.len()-1].1);
   
    ordered.pop();
    ordered.pop();

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
