#[macro_use]
extern crate clap;

use clap::App;

use lazy_static::lazy_static;
use rust_v::configuration::{Configuration, DemoType, IntegratorType, PixelType};
use std::convert::TryInto;

const LIVE: &str = "LIVE_WINDOW";
const SPHERES: &str = "spheres";
const CORNELL: &str = "cornell";
const VERBOSE: &str = "VERBOSE";
#[allow(dead_code)]
const INPUT: &str = "INPUT";
const OUTPUT: &str = "OUTPUT";
const PASSES: &str = "PASSES";
const BLOCK_SIZE: &str = "BLOCK_SIZE";
const DEPTH: &str = "DEPTH";
const WIDTH: &str = "WIDTH";
const HEIGHT: &str = "HEIGHT";
const FORMAT: &str = "FORMAT";
const INTEGRATOR_BACKEND: &str = "INTEGRATOR_BACKEND";
const THREADS: &str = "THREADS";

lazy_static! {
    static ref CONFIG: Configuration = {
        #[cfg(not(feature = "live-window"))]
        let yaml = load_yaml!("cli.yml");
        #[cfg(feature = "live-window")]
        let yaml = load_yaml!("cli-live.yml");

        let app_matches = App::from_yaml(yaml).get_matches();

        let demo = if let Some(spheres) = app_matches.subcommand_matches(SPHERES) {
            (spheres, DemoType::Spheres)
        } else if let Some(cornell) = app_matches.subcommand_matches(CORNELL) {
            (cornell, DemoType::Cornell)
        } else {
            panic!("Currently we only support the subcommands (spheres, cornell)!");
        };

        let matches = demo.0;

        let verbose = matches.is_present(VERBOSE);
        let width = match matches.value_of(WIDTH).unwrap_or("900").parse() {
            Ok(width) => width,
            Err(err) => panic!("Cannot parse width: {}", err),
        };
        let height = match matches.value_of(HEIGHT).unwrap_or("900").parse() {
            Ok(height) => height,
            Err(err) => panic!("Cannot parse height: {}", err),
        };
        let depth = match matches.value_of(DEPTH).unwrap_or("6").parse() {
            Ok(depth) => depth,
            Err(err) => panic!("Cannot parse depth: {}", err),
        };
        let passes = match matches.value_of(PASSES).unwrap_or("1").parse() {
            Ok(passes) => passes,
            Err(err) => panic!("Cannot parse passes: {}", err),
        };
        let block_size = match matches.value_of(BLOCK_SIZE).unwrap_or("8").parse() {
            Ok(block_size) => block_size,
            Err(err) => panic!("Cannot parse block size: {}", err),
        };
        let live = cfg!(feature = "live-window") && matches.is_present(LIVE);
        let threads = match matches
            .value_of(THREADS)
            .unwrap_or(&num_cpus::get().to_string())
            .parse()
        {
            Ok(threads) => threads,
            Err(err) => panic!("Cannot parse threads: {}", err),
        };
        let pixel_type: PixelType = match matches.value_of(FORMAT).unwrap_or("U8").try_into() {
            Ok(format) => format,
            Err(err) => panic!("Cannot parse pixel format: {}", err),
        };
        let integrator_type: IntegratorType = match matches
            .value_of(INTEGRATOR_BACKEND)
            .unwrap_or("whitted")
            .try_into()
        {
            Ok(integrator) => integrator,
            Err(err) => panic!("Cannot parse integrator backend: {}", err),
        };

        let output = if let Some(o) = matches.value_of(OUTPUT) {
            o.to_string()
        } else {
            format!(
                "{}.png",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )
        };
        let output = if output.is_empty() {
            None
        } else {
            Some(output)
        };

        let demo_type = demo.1;

        Configuration {
            verbose,
            width,
            height,
            depth,
            passes,
            block_size,
            live,
            threads,
            output,
            pixel_type,
            integrator_type,
            demo_type,
        }
    };
}

fn main() -> Result<(), String> {
    CONFIG.start_rendering()
}
