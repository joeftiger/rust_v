#[macro_use]
extern crate clap;

use clap::App;

use rust_v::configuration::{Configuration, IntegratorBackend, PixelFormat};
use std::convert::TryInto;
use lazy_static::lazy_static;

const LIVE: &str = "LIVE_WINDOW";
const DEMO: &str = "demo";
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
const THREADED: &str = "THREADED";

lazy_static! {
    static ref CONFIG: Configuration = {
        #[cfg(not(feature = "live-window"))]
        let yaml = load_yaml!("cli.yml");
        #[cfg(feature = "live-window")]
        let yaml = load_yaml!("cli-live.yml");

        let matches = App::from_yaml(yaml).get_matches();

        if let Some(demo) = matches.subcommand_matches(DEMO) {
            let verbose = demo.is_present(VERBOSE);
            let width = match demo.value_of(WIDTH).unwrap_or("900").parse() {
                Ok(width) => width,
                Err(err) => panic!("Cannot parse width: {}", err),
            };
            let height = match demo.value_of(HEIGHT).unwrap_or("900").parse() {
                Ok(height) => height,
                Err(err) => panic!("Cannot parse height: {}", err),
            };
            let depth = match demo.value_of(DEPTH).unwrap_or("20").parse() {
                Ok(depth) => depth,
                Err(err) => panic!("Cannot parse depth: {}", err),
            };
            let passes = match demo.value_of(PASSES).unwrap_or("1").parse() {
                Ok(passes) => passes,
                Err(err) => panic!("Cannot parse passes: {}", err),
            };
            let block_size = match demo.value_of(BLOCK_SIZE).unwrap_or("8").parse() {
                Ok(block_size) => block_size,
                Err(err) => panic!("Cannot parse block size: {}", err),
            };
            let live = cfg!(feature = "live-window") && demo.is_present(LIVE);
            let threaded = demo.is_present(THREADED);
            let pixel_format: PixelFormat = match demo.value_of(FORMAT).unwrap_or("U8").try_into() {
                Ok(format) => format,
                Err(err) => panic!("Cannot parse pixel format: {}", err),
            };
            let integrator_backend: IntegratorBackend = match demo
                .value_of(INTEGRATOR_BACKEND)
                .unwrap_or("whitted")
                .try_into()
            {
                Ok(integrator) => integrator,
                Err(err) => panic!("Cannot parse integrator backend: {}", err),
            };

            let output = if let Some(o) = demo.value_of(OUTPUT) {
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

            Configuration::new(
                verbose,
                width,
                height,
                depth,
                passes,
                block_size,
                live,
                threaded,
                output,
                pixel_format,
                integrator_backend,
            )
        } else {
            panic!("Currently we only support the demo subcommand!");
        }
    };
}



fn main() -> Result<(), String> {
    CONFIG.start_rendering()
}
