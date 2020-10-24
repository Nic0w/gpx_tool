extern crate serde;
extern crate quick_xml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::hash::{Hash, Hasher};

use log::{info, warn, trace};

use serde::Deserialize;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("ele")]
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Elevation {
    #[serde(rename = "$value")]
    #[sxs_type_text]
    pub value: f32
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
pub struct Name {
    #[serde(rename = "$value")]
    #[sxs_type_text]
    pub value: String
}

#[xml_element("trkpt")]
#[derive(Debug, Deserialize)]
pub struct TrackPoint {
    #[sxs_type_attr]
    pub lat: f64,

    #[sxs_type_attr]
    pub lon: f64,

    #[serde(rename = "ele", default)]
    #[sxs_type_multi_element(rename="ele")]
    pub elevations: Vec<Elevation>
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
pub struct TrackSegment {
    #[serde(rename = "trkpt", default)]
    #[sxs_type_multi_element(rename="trkpt")]
    pub points: Vec<TrackPoint>
}

#[xml_element("trk")]
#[derive(Debug, Deserialize, Hash)]
pub struct Track {
   #[sxs_type_element]
   pub name: Name,

   #[serde(rename = "trkseg", default)]
   #[sxs_type_multi_element(rename="trkseg")]
   pub segments: Vec<TrackSegment>
}

#[xml_element("metadata")]
#[derive(Debug, Deserialize, PartialEq, Hash)]
pub struct Metadata {
    #[sxs_type_element]
    pub name: Name
}

#[xml_element("gpx")]
#[derive(Debug, Deserialize, Hash)]
pub struct Gpx {
    #[sxs_type_attr]
    pub creator: String,

    #[sxs_type_attr]
    pub version: String,

    #[sxs_type_element]
    pub metadata: Metadata,

    #[serde(rename = "trk", default)]
    #[sxs_type_multi_element(rename="trk")]
    pub tracks: Vec<Track>
}

pub fn parse_gpx(path: &Path) -> Gpx {

    match File::open(&path) {
        Err(reason) => panic!("Failed to open file: {}", reason),
        Ok(mut gpx_file)    => {

            let mut content = String::new();

            match gpx_file.read_to_string(&mut content) {
                Err(reason) => panic!("Failed to read file: {}", reason),
                Ok(size) => trace!("Read {} bytes !", size)
            };

            match quick_xml::de::from_str(&content) {

                Err(reason) => panic!("Failed to parse data: {}", reason),
                Ok(data) => data
            }
        }
    }
}

pub fn point2trackpoint(point: &(f64, f64)) -> TrackPoint {

    TrackPoint {
        lat: point.0,
        lon: point.1,

        elevations: vec![]
    }
}

pub fn trackpoint2point(trackpoint: &TrackPoint) -> (f64, f64) {

    (trackpoint.lat, trackpoint.lon)
}

pub fn to_gpx(points: &Vec<(f64, f64)>, path: Option<&Path>, name: Option<&str>, creator: Option<&str>, version: Option<&str>) {

    let name = name.unwrap_or("Some GPX Data");
    let creator = creator.unwrap_or("gpx_tool");
    let version = version.unwrap_or("1.1");

    let gpx_object = Gpx {
        creator: creator.to_string(),
        version: version.to_string(),

        metadata: Metadata {
            name: Name { value: name.to_string() }
        },

        tracks: vec![Track {
            name: Name { value: name.to_string() },
            segments: vec![TrackSegment{
                points: points.iter().map(point2trackpoint).collect(),
            }]
        }]
    };

    let xml_data = XMLElement::from(gpx_object).to_string_pretty("\n", "  ");

    if let Some(path) = path {

        let _res = match File::create(path) {

            Ok(mut file) => file.write_all(xml_data.as_bytes()),

            Err(_reason) => panic!("Failed to open a file for writing."),
        };

    }
    else {
        println!("{}", xml_data);
    }

}
