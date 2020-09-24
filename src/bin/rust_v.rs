extern crate clap;

use clap::{App, Arg};
use std::time::Instant;
use rust_v::geometry::sphere::Sphere;
use ultraviolet::Vec3;
use rust_v::geometry::{Ray, Intersectable};

const INPUT: &str = "input_file";
const OUTPUT: &str = "output_file";
const VERBOSE: &str = "VERBOSE";

fn main() {
    // let app = init_help();
    // let _matches = app.get_matches();

   quick_bench();
}

fn quick_bench() {
   const SECONDS: u64 = 300;

   let sphere = Sphere::new(Vec3::zero(), 1.0);
   let ray = Ray::new(-Vec3::unit_x() * 2.0, Vec3::unit_x());

   let now = Instant::now();

   println!("Warming up for {} s...", 5);
   while now.elapsed().as_secs() < 5 {
      let _hit = sphere.intersects(&ray);
   }

   println!("Benchmarking for {} s...", SECONDS);;
   let mut hits = 0;
   let now = Instant::now();

   while now.elapsed().as_secs() < SECONDS {
      let _hit = sphere.intersects(&ray);
      hits += 1;
   }

   println!("{} casts/s", hits / SECONDS);
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
