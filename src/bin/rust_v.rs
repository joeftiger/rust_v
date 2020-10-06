extern crate clap;

use std::time::Instant;

use clap::{App, Arg};
use ultraviolet::Vec3;
use serde_json::json;

use rust_v::geometry::{Intersectable, Ray};
use rust_v::geometry::aabb::Aabb;
use std::fs::{File, Permissions, OpenOptions, remove_dir, create_dir, remove_file};
use std::io::{Write, Read};

const INPUT: &str = "input_file";
const OUTPUT: &str = "output_file";
const VERBOSE: &str = "VERBOSE";

fn main() {
    // let app = init_help();
    // let _matches = app.get_matches();

    // quick_bench();
    test_save_load_aabb();
}

fn test_save_load_aabb() {
    let aabb = Aabb::new(-Vec3::one(), Vec3::one());
    let json = serde_json::to_string_pretty(&aabb).expect("Could not serialize aabb");

    {
        create_dir("./json_tests").expect("Could not create test directory");

        OpenOptions::new()
            .create(true)
            .write(true)
            .open("./json_tests/aabb.json")
            .unwrap()
            .write_all(json.as_ref())
            .expect("Could not write to test file");
    }

    let mut json_read = String::new();

    {
        OpenOptions::new()
            .read(true)
            .open("./json_tests/aabb.json")
            .unwrap()
            .read_to_string(&mut json_read)
            .expect("Could not read from test file");
    }

    assert_eq!(json_read, json);

    let aabb_read: Aabb = serde_json::from_str(&*json_read).expect("Could not parse aabb");

    assert_eq!(aabb.min, aabb_read.min);
    assert_eq!(aabb.max, aabb_read.max);

    remove_file("./json_tests/aabb.json").expect("Could not delete test file");
    remove_dir("./json_tests").expect("Could not delete test directory");
}

fn quick_bench() {
    const SECONDS: u64 = 10;

    let now = Instant::now();

    println!("Warming up for 5 s...");
    while now.elapsed().as_secs() < 5 {
        let min = -Vec3::new(fastrand::f32(), fastrand::f32(), rand::random());
        let max = Vec3::new(fastrand::f32(), fastrand::f32(), rand::random());
        let aabb = Aabb::new(min, max);

        let origin = -Vec3::new(fastrand::f32(), fastrand::f32(), rand::random()) * 2.0;
        let direction = Vec3::new(fastrand::f32(), fastrand::f32(), rand::random());
        let ray = Ray::new(origin, direction);

        let _hit = aabb.intersects(ray).is_some();
    }

    println!("Benchmarking for {} s...", SECONDS);
    let mut hits: u64 = 0;
    let mut casts: u64 = 0;
    let now = Instant::now();

    while now.elapsed().as_secs() < SECONDS {
        let min = -Vec3::new(fastrand::f32(), fastrand::f32(), rand::random());
        let max = Vec3::new(fastrand::f32(), fastrand::f32(), rand::random());
        let aabb = Aabb::new(min, max);

        let origin = -Vec3::new(fastrand::f32(), fastrand::f32(), rand::random()) * 2.0;
        let direction = Vec3::new(fastrand::f32(), fastrand::f32(), rand::random());

        let ray = Ray::new(origin, direction);
        if aabb.intersects(ray).is_some() {
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
