extern crate serde;
extern crate quick_xml;

use std::hash::{Hash, Hasher};

use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::{Deserialize, Serialize};
use quick_xml::de::{from_str, DeError};
use quick_xml::se::to_string;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
struct Elevation {
    #[serde(rename = "$value")]
    value: f32
}

impl PartialEq for Elevation {
    fn eq(&self, other: &Self) -> bool {

        self.value.to_bits() == other.value.to_bits()
    }
}
impl Eq for Elevation {}

impl Hash for Elevation {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Hash)]
struct Name {
    #[serde(rename = "$value")]
    value: String
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackPoint {
    lat: f64,
    lon: f64,

    #[serde(rename = "ele", default)]
    elevations: Vec<Elevation>
}

impl Clone for TrackPoint {

    fn clone(&self) -> TrackPoint {
        TrackPoint {
            lat: self.lat,
            lon: self.lon,
            elevations: self.elevations.clone()
        }
    }
}
//impl Copy for TrackPoint {}

impl PartialEq for TrackPoint {
    fn eq(&self, other: &Self) -> bool {
        
        self.lat.to_bits() == other.lat.to_bits() &&
            self.lon.to_bits() == other.lon.to_bits() /*&&
                self.elevations[0].eq(&other.elevations[0])*/
    }
}
impl Eq for TrackPoint {}

impl Hash for TrackPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
    
        self.lat.to_bits().hash(state);
        self.lon.to_bits().hash(state);

        //self.elevations[0].hash(state);
    }
}

#[derive(Debug, Deserialize, Serialize, Hash)]
struct TrackSegment {
    #[serde(rename = "trkpt", default)]
    points: Vec<TrackPoint>
}

#[derive(Debug, Deserialize, Serialize, Hash)]
struct Track {
   name: Name,

   #[serde(rename = "trkseg", default)]
   segments: Vec<TrackSegment>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Hash)]
struct Metadata {
    name: Name
}

#[derive(Debug, Deserialize, Serialize, Hash)]
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

    let gpx: Gpx = match from_str(&content) {
    
        Err(reason) => panic!("Failed to parse data: {}", reason),
        Ok(data) => data
    };

    println!("File by '{}', version {}", gpx.creator, gpx.version);
    println!("Parsed {} tracks !", gpx.tracks.len());

    let mut unique_points: HashSet<TrackPoint> = HashSet::new();
    let mut final_points: Vec<TrackPoint> = Vec::new();
    let mut point_count = 0usize;
    for (i, track) in gpx.tracks.iter().enumerate() {
        println!("{} segments in track {}.", track.segments.len(), i);
        
        for segment in track.segments.iter() { 
            println!("\t{} points in segment.", segment.points.len());

            for point in segment.points.iter() {
            
                if unique_points.insert(point.clone()) {
                    final_points.push(point.clone());
                }
            }
        
        }

        point_count += track.segments[0].points.len()
    }

    println!("{} points in total, kept {}.", point_count, final_points.len());

    /*let simple_vec = |elt|{
        let vec = Vec::new();
        vec.push(elt);
        vec
    };*/


    /*let segment = simple_vec(TrackSegment{
        points: final_points.clone()
    });

    let track = simple_vec(Track {
        name: Name { value: "TestSeg".to_string() },
        segments: segment
    })*/ 

    let cleanedup_gpx = Gpx {
        creator: "nic0w".to_string(),
        version: "1.1".to_string(),

        metadata: Metadata {
            name: Name { value: "Test".to_string() }
        },

        tracks: (|track| { let mut vec = Vec::new(); vec.push(track); vec})(Track {
            name: Name { value: "TestSeg".to_string() },
            segments: (|seg| { let mut vec = Vec::new(); vec.push(seg); vec})(TrackSegment{
                points: final_points.clone()
            })
        })
    };

    let result =  match to_string(&cleanedup_gpx) {
        Err(reason) => panic!("Failed to serialized: {}", reason),
        Ok(content) => content
    };

    println!("{}", result);
}
