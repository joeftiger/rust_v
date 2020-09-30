extern crate clap;

use std::time::Instant;

use clap::{App, Arg};
use ultraviolet::Vec3;

use rust_v::geometry::{Intersectable, Ray};
use rust_v::geometry::aabb::Aabb;

const INPUT: &str = "input_file";
const OUTPUT: &str = "output_file";
const VERBOSE: &str = "VERBOSE";

fn main() {
    // let app = init_help();
    // let _matches = app.get_matches();

    quick_bench();
}

fn quick_bench() {
    const SECONDS: u64 = 10;

    let now = Instant::now();

    println!("Warming up for 5 s...");
    while now.elapsed().as_secs() < 5 {
        let min = -Vec3::new(rand::random(), rand::random(), rand::random());
        let max = Vec3::new(rand::random(), rand::random(), rand::random());
        let aabb = Aabb::new(min, max);

        let origin = -Vec3::new(rand::random(), rand::random(), rand::random()) * 2.0;
        let direction = Vec3::new(rand::random(), rand::random(), rand::random());
        let ray = Ray::new(origin, direction);

        let _hit = aabb.intersects(&ray).is_some();
    }

    println!("Benchmarking for {} s...", SECONDS);
    let mut hits: u64 = 0;
    let mut casts: u64 = 0;
    let now = Instant::now();

    while now.elapsed().as_secs() < SECONDS {
        let min = -Vec3::new(rand::random(), rand::random(), rand::random());
        let max = Vec3::new(rand::random(), rand::random(), rand::random());
        let aabb = Aabb::new(min, max);

        let origin = -Vec3::new(rand::random(), rand::random(), rand::random()) * 2.0;
        let direction = Vec3::new(rand::random(), rand::random(), rand::random());

        let ray = Ray::new(origin, direction);
        if aabb.intersects(&ray).is_some() {
            hits += 1;
        }
        casts += 1;
    }

    println!("{} hits", hits);
    println!("{} casts/s", casts / SECONDS);
}

fn init_help<'a, 'b>() -> App<'a, 'b> {
    App::new("Rust-V")
        .version("0.0.1")
        .author("Julius Oeftiger")
        .about("A rust ray tracer supporting rgb and spectral ray tracing")
        .arg(
            Arg::with_name(INPUT)
                // .short("i")
                // .long("input")
                .help("The input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name(OUTPUT)
                // .short("o")
                // .long("output")
                .help("The output file to save in (png)")
                .required(true),
        )
        .arg(
            Arg::with_name(VERBOSE)
                .short("v")
                .long("verbose")
                .required(false),
        )
}
