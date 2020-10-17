extern crate serde;
extern crate quick_xml;

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

    let mut i = 0u32;

    let ret = loop {
    
        println!("{}", i);

        if i > 50 {
        
            break i%12;
        }

        i += match i%9 {
        
            0       => 1,
            1..=3    => 2,
            4|6|8   => 3,
            _       => 4
        
        }
    };

    println!("Final value: {}", ret);

    let ys: [i32; 500] = [5; 500];

    println!("{:?} {:?}", ys[0], ys[1]);
}
