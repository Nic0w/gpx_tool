extern crate serde;
extern crate quick_xml;

use std::f64::consts::PI;

use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use simple_xml_serialize::XMLElement;

use quick_xml::de::{from_str, DeError};


mod gpx;
use crate::gpx::*;

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
        //println!("{} segments in track {}.", track.segments.len(), i);
        
        for segment in track.segments.iter() { 
            //println!("\t{} points in segment.", segment.points.len());

            for point in segment.points.iter() {
           
                let current = (point.lat, point.lon);
                let dist = distance(previous, current);
                previous = current;

                if unique_points.insert(point.clone()) {
                    final_points.push(point.clone());
                }
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
