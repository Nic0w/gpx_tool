extern crate serde;
extern crate quick_xml;
extern crate clap;

use clap::{App, Arg, AppSettings};
use log::{info, warn, trace};

mod repair;
mod lookup;
mod gpx;
mod tsp;
mod distance;
mod combine;
mod info;

use crate::repair::repair;
use crate::combine::combine;
use crate::info::info;

fn main() {

    let app = App::new("GPX Parser").
        setting(AppSettings::ArgRequiredElseHelp).
        version("1.0").
        author("nic0w").
        about("Fixes or combines (broken) gpx files").
        arg(Arg::with_name("quiet").
            short("q").
            long("quiet").
            help("Disable messages.")).
        arg(Arg::with_name("verbosity").
                short("v").
                long("verbose").
                multiple(true).
                help("Increases verbosity.")).
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
            arg(Arg::with_name("output").
                short("o").
                long("output").
                value_name("file").
                help("Ouput file").
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
        subcommand(App::new("info").
            about("Informations on the structure of the file").
            arg(Arg::with_name("file").
                required(true).
                help("file to print infos on."))).
        get_matches();

    stderrlog::new()
            .module(module_path!())
            .quiet(app.is_present("quiet"))
            .timestamp(stderrlog::Timestamp::Second)
            .verbosity((app.occurrences_of("verbosity")+1u64) as usize)
            .init()
            .unwrap();

    info!("Hello, world!");

    match app.subcommand() {

        ("repair", Some(args)) => {
            repair(
                args.value_of("file"),
                args.value_of("output"),
                args.value_of("cardinal-point"),
                args.value_of("start-point"),
                args.value_of("truncate")
            );
        },
        ("combine", Some(args)) => {
            combine(args.values_of("file").unwrap().collect());
        },
        ("info", Some(args)) => {
            info(args.value_of("file"));
        },
        _ => {}
    };

    //(48.947646f64, 2.153013f64)
}
