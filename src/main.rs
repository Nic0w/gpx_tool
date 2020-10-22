extern crate serde;
extern crate quick_xml;
extern crate clap;

use clap::{App, Arg};

mod repair;
mod lookup;
mod gpx;
mod tsp;
mod distance;

use crate::repair::repair;

fn main() {

    let app = App::new("GPX Parser").
        version("1.0").
        author("nic0w").
        about("Fixes or combines (broken) gpx files").
        subcommand(App::new("repair").
            about("Repair a gpx file").
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
            arg(Arg::with_name("truncate").
                short("t").
                long("truncate").
                value_name("number").
                help("Will remove number of points from the end of the repaired track. Can help with outliers.").
                takes_value(true)).
            arg(Arg::with_name("file").
                value_name("./path/to/file.gpx").
                help("File to fix").
                takes_value(true).
                required(true))).
        get_matches();

    println!("Hello, world!");

    match app.subcommand() {

        ("repair", Some(args)) => {
            repair(
                args.value_of("file"),
                args.value_of("cardinal-point"),
                args.value_of("start-point")
            );
        }
        _ => {}
    };

    /*//(48.947646f64, 2.153013f64)


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

    let truncate: usize = match args.value_of("truncate") {
        Some(n) => n.parse().unwrap(),
        _       => 0usize
    };

    for _i in 0..truncate {
        ordered.pop();
    }

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

    let output = Path::new("output.gpx");

    let mut out_file = match File::create(output) {

        Ok(file) => file,
        Err(reason) => panic!("Failed to open a file for writing."),

    };

    out_file.write_all(cleanedup_xml.to_string_pretty("\n", "  ").as_bytes());

    println!("{}", cleanedup_xml.to_string_pretty("\n", "  "));*/
}
