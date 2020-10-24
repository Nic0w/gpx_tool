extern crate serde;
extern crate quick_xml;
extern crate clap;

use clap::{App, Arg};

mod repair;
mod lookup;
mod gpx;
mod tsp;
mod distance;
mod combine;

use crate::repair::repair;
use crate::combine::combine;

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
        subcommand(App::new("combine").
            about("Combine multiple GPX files into one.").
            arg(Arg::with_name("file").
                multiple(true).
                required(true).
                help("List of files to combine")).
            arg(Arg::with_name("repair").
                short("r").
                long("repair").
                help("Apply the repair algorithm on the combined data"))).
        get_matches();

    println!("Hello, world!");

    match app.subcommand() {

        ("repair", Some(args)) => {
            repair(
                args.value_of("file"),
                args.value_of("cardinal-point"),
                args.value_of("start-point"),
                args.value_of("truncate")
            );
        },
        ("combine", Some(args)) => {
            combine(args.values_of("file").unwrap().collect());
        },
        _ => {}
    };

    //(48.947646f64, 2.153013f64)
}
