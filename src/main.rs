extern crate serde;
extern crate quick_xml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};

#[derive(Debug, Deserialize, PartialEq)]
struct Elevation {
    #[serde(rename = "$value")]
    value: f32
}

#[derive(Debug, Deserialize, PartialEq)]
struct Name {
    #[serde(rename = "$value")]
    value: String
}

#[derive(Debug, Deserialize, PartialEq)]
struct TrackPoint {
    lat: f64,
    lon: f64,

    #[serde(rename = "ele", default)]
    elevations: Vec<Elevation>
}

#[derive(Debug, Deserialize, PartialEq)]
struct TrackSegment {
    #[serde(rename = "trkpt", default)]
    points: Vec<TrackPoint>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Track {
   name: Name,

   #[serde(rename = "trkseg", default)]
   segments: Vec<TrackSegment>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Metadata {
    name: Name
}

#[derive(Debug, Deserialize, PartialEq)]
struct Gpx {
    creator: String,
    version: String,

    metadata: Metadata,
    
    #[serde(rename = "trk", default)]
    tracks: Vec<Track>
}

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

}
