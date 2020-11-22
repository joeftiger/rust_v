#[macro_use]
extern crate clap;

use clap::App;

use indicatif::{ProgressBar, ProgressStyle};
use rust_v::cornell_box;
use rust_v::render::integrator::debug_normals::DebugNormals;
use rust_v::render::integrator::whitted::Whitted;
use rust_v::render::integrator::Integrator;
use rust_v::render::renderer::Renderer;
use rust_v::render::sampler::RandomSampler;
use rust_v::render::window::RenderWindow;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};

const LIVE: &str = "LIVE_WINDOW";
const DEMO: &str = "demo";
const VERBOSE: &str = "VERBOSE";
const DEBUG: &str = "DEBUG";
#[allow(dead_code)]
const INPUT: &str = "INPUT";
const OUTPUT: &str = "OUTPUT";
const PASSES: &str = "PASSES";
const DEPTH: &str = "DEPTH";
const WIDTH: &str = "WIDTH";
const HEIGHT: &str = "HEIGHT";
const FORMAT: &str = "FORMAT";

fn main() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml).get_matches();

    if let Some(demo) = matches.subcommand_matches(DEMO) {
        let verbose = demo.is_present(VERBOSE);
        let debug = demo.is_present(DEBUG);
        let width = match demo.value_of(WIDTH).unwrap_or("900").parse() {
            Ok(width) => width,
            Err(err) => panic!("Cannot parse width: {}", err),
        };
        let height = match demo.value_of(HEIGHT).unwrap_or("900").parse() {
            Ok(height) => height,
            Err(err) => panic!("Cannot parse height: {}", err),
        };
        let depth = match demo.value_of(DEPTH).unwrap_or("6").parse() {
            Ok(depth) => depth,
            Err(err) => panic!("Cannot parse depth: {}", err),
        };
        let passes = match demo.value_of(PASSES).unwrap_or("1").parse() {
            Ok(passes) => passes,
            Err(err) => panic!("Cannot parse passes: {}", err),
        };
        let live = demo.is_present(LIVE);
        let pixel_format = match demo.value_of(FORMAT).unwrap_or("u8").try_into() {
            Ok(format) => format,
            Err(err) => panic!("Cannot parse pixel format: {}", err),
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
        let output = output.as_str();

        let mut main = Main::new(
            verbose,
            debug,
            width,
            height,
            depth,
            passes,
            live,
            output,
            pixel_format,
        );
        main.start()
    } else {
        Err("Currently we only support the demo subcommand!".to_string())
    }
}

#[derive(Debug)]
struct Main<'a> {
    verbose: bool,
    debug: bool,
    width: u32,
    height: u32,
    depth: u32,
    passes: u32,
    live: bool,
    output: &'a str,
    pixel_format: PixelFormat,
}

impl<'a> Main<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        verbose: bool,
        debug: bool,
        width: u32,
        height: u32,
        depth: u32,
        passes: u32,
        live: bool,
        output: &'a str,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            verbose,
            debug,
            width,
            height,
            depth,
            passes,
            live,
            output,
            pixel_format,
        }
    }

    fn start(&mut self) -> Result<(), String> {
        if self.verbose {
            println!("{:#?}", self);
        }

        let (scene, camera) = cornell_box::create(self.width, self.height);
        let sampler = Mutex::new(RandomSampler);
        let integrator: Arc<dyn Integrator> = if self.debug {
            Arc::new(DebugNormals)
        } else {
            Arc::new(Whitted::new(self.depth))
        };
        let mut renderer = Renderer::new(
            Arc::new(scene),
            Arc::new(camera),
            Arc::new(sampler),
            integrator,
        );

        if self.live {
            RenderWindow::new("Rust-V".to_string(), &mut renderer)
                .map_err(|e| format!("Unable to create window: {}", e))?
                .start_rendering();
            Ok(())
        } else {
            let bar = {
                let bar = ProgressBar::new(renderer.num_pixels() as u64);
                bar.set_style(ProgressStyle::default_bar().template(
                    "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining]",
                ));

                Arc::new(Mutex::new(bar))
            };

            // renderer.render_all(self.passes, bar.clone());
            renderer.render_all_par(self.passes, bar.clone());
            bar.lock().expect("ProgressBar poisoned").finish();

            match self.pixel_format {
                PixelFormat::u8 => renderer
                    .get_image_u8()
                    .save(&self.output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
                PixelFormat::u16 => renderer
                    .get_image_u16()
                    .save(&self.output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
            };

            Ok(())
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum PixelFormat {
    u8,
    u16,
}

impl TryInto<PixelFormat> for &str {
    type Error = String;

    fn try_into(self) -> Result<PixelFormat, Self::Error> {
        match self {
            "u8" => Ok(PixelFormat::u8),
            "u16" => Ok(PixelFormat::u16),
            _ => Err(self.to_string()),
        }
    }
}
