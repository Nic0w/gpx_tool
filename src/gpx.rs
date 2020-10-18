extern crate serde;
extern crate quick_xml;

use std::hash::{Hash, Hasher};

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