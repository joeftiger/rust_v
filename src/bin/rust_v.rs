extern crate clap;

use clap::{App, Arg};

const INPUT: &str = "input_file";
const OUTPUT: &str = "output_file";
const VERBOSE: &str = "VERBOSE";

fn main() {
    init_help();
}

fn init_help<'a, 'b>() -> App<'a, 'b> {
    App::new("Rust-V")
        .version("0.0.1")
        .author("Julius Oeftiger")
        .about("A rust ray tracer supporting rgb and spectral ray tracing")
        .arg(Arg::with_name(INPUT)
            // .short("i")
            // .long("input")
            .help("The input file to use")
            .required(true))
        .arg(Arg::with_name(OUTPUT)
            // .short("o")
            // .long("output")
            .help("The output file to save in (png)")
            .required(true))
        .arg(Arg::with_name(VERBOSE)
            .short("v")
            .long("verbose")
            .required(false))
}
