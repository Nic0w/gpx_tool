extern crate serde;
extern crate quick_xml;

use std::f64::consts::PI;

use std::hash::{Hash, Hasher};

use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
//use quick_xml::se::to_string;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("ele")]
#[derive(Debug, Deserialize, Copy, Clone)]
struct Elevation {
    #[serde(rename = "$value")]
    #[sxs_type_text]
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

#[xml_element("name")]
#[derive(Debug, Deserialize, PartialEq, Hash)]
struct Name {
    #[serde(rename = "$value")]
    #[sxs_type_text]
    value: String
}

#[xml_element("trkpt")]
#[derive(Debug, Deserialize)]
struct TrackPoint {
    #[sxs_type_attr]
    lat: f64,

    #[sxs_type_attr]
    lon: f64,

    #[serde(rename = "ele", default)]
    #[sxs_type_multi_element(rename="ele")]
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

#[xml_element("trkseg")]
#[derive(Debug, Deserialize, Hash)]
struct TrackSegment {
    #[serde(rename = "trkpt", default)]
    #[sxs_type_multi_element(rename="trkpt")]
    points: Vec<TrackPoint>
}

#[xml_element("trk")]
#[derive(Debug, Deserialize, Hash)]
struct Track {
   #[sxs_type_element]
   name: Name,

   #[serde(rename = "trkseg", default)]
   #[sxs_type_multi_element(rename="trkseg")]
   segments: Vec<TrackSegment>
}

#[xml_element("metadata")]
#[derive(Debug, Deserialize, PartialEq, Hash)]
struct Metadata {
    #[sxs_type_element]
    name: Name
}

#[xml_element("gpx")]
#[derive(Debug, Deserialize, Hash)]
struct Gpx {
    #[sxs_type_attr]
    creator: String,

    #[sxs_type_attr]
    version: String,

    #[sxs_type_element]
    metadata: Metadata,
    
    #[serde(rename = "trk", default)]
    #[sxs_type_multi_element(rename="trk")]
    tracks: Vec<Track>
}

const EARTH_RADIUS: f64 = 6371e3;

fn distance(coords_1: (f64, f64), coords_2: (f64, f64)) -> f64 {

    let (lat1, lon1) = coords_1;
    let (lat2, lon2) = coords_2;

//  println!("Computing distance between ({}, {}) and ({}, {}).", lat1, lon1, lat2, lon2);

    let to_radians = |x| x * (PI/180.0);

    let phi1 = to_radians(lat1);
    let phi2 = to_radians(lat2);

    //let d_phi = mul_by_pi_frac_180(if lat1 > lat2 {lat1-lat2} else {lat2-lat1});
    //let d_del = mul_by_pi_frac_180(if lon1 > lon2 {lon1-lon2} else {lon2-lon1});

    let d_phi = to_radians(lat2-lat1);
    let d_del = to_radians(lon2-lon1);

//  println!("{} {} {} {}", phi1, phi2, d_phi, d_del);

    let a = (d_phi/2.0).sin().powi(2) + phi1.cos()*phi2.cos()*(d_del/2.0).sin().powi(2);

//  println!("a {}", a);

    let c = a.sqrt().atan2((1.0-a).sqrt()) * 2.0;

    EARTH_RADIUS * c
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
    let mut previous = (std::f64::NAN, std::f64::NAN);
    let mut avg_dist: f64 = 50.0; 
    let mut outlier: bool = false;
    for (i, track) in gpx.tracks.iter().enumerate() {
        println!("{} segments in track {}.", track.segments.len(), i);
        
        for segment in track.segments.iter() { 
            println!("\t{} points in segment.", segment.points.len());

            for point in segment.points.iter() {
           
                let current = (point.lat, point.lon);
                let dist = distance(previous, current);
                previous = current;

               // println!("sqrt {}", dist.sqrt());
                if dist > 200.0 { println!("outlier? {}", dist); outlier=true; }

                if !dist.is_nan() && !outlier {  
                  //  println!("dist {} avg {} pc {}", dist, avg_dist, point_count);
                    avg_dist = (avg_dist + dist) / 2.0;
                }

                //println!("\t\tDistance with previous point: {} meters; avg={}", dist, avg_dist);

                if unique_points.insert(point.clone()) && !outlier {
                    final_points.push(point.clone());
                }

                outlier=false;
            }
        
        }

        point_count += track.segments[0].points.len()
    }

    println!("{} points in total, kept {}.", point_count, final_points.len());

    let cleanedup_gpx = Gpx {
        creator: "nic0w".to_string(),
        version: "1.1".to_string(),

        metadata: Metadata {
            name: Name { value: "Test".to_string() }
        },

        tracks: vec![Track {
            name: Name { value: "TestSeg".to_string() },
            segments: vec![TrackSegment{
                points: final_points.clone()
            }]
        }]
    };

    let cleanedup_xml = XMLElement::from(cleanedup_gpx);

    println!("{}", cleanedup_xml.to_string_pretty("\n", "  "));
}
