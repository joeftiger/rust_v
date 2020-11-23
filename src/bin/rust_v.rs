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
use rust_v::render::integrator::path::Path;

const LIVE: &str = "LIVE_WINDOW";
const DEMO: &str = "demo";
const VERBOSE: &str = "VERBOSE";
#[allow(dead_code)]
const INPUT: &str = "INPUT";
const OUTPUT: &str = "OUTPUT";
const PASSES: &str = "PASSES";
const DEPTH: &str = "DEPTH";
const WIDTH: &str = "WIDTH";
const HEIGHT: &str = "HEIGHT";
const FORMAT: &str = "FORMAT";
const INTEGRATOR_BACKEND: &str = "INTEGRATOR_BACKEND";

fn main() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");

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
        let depth = match demo.value_of(DEPTH).unwrap_or("6").parse() {
            Ok(depth) => depth,
            Err(err) => panic!("Cannot parse depth: {}", err),
        };
        let passes = match demo.value_of(PASSES).unwrap_or("1").parse() {
            Ok(passes) => passes,
            Err(err) => panic!("Cannot parse passes: {}", err),
        };
        let live = demo.is_present(LIVE);
        let pixel_format = match demo.value_of(FORMAT).unwrap_or("U8").try_into() {
            Ok(format) => format,
            Err(err) => panic!("Cannot parse pixel format: {}", err),
        };
        let integrator_backend = match demo.value_of(INTEGRATOR_BACKEND).unwrap_or("whitted").try_into() {
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
        let output = output.as_str();

        let mut main = Configuration::new(
            verbose,
            width,
            height,
            depth,
            passes,
            live,
            output,
            pixel_format,
            integrator_backend
        );
        main.start()
    } else {
        Err("Currently we only support the demo subcommand!".to_string())
    }
}

#[derive(Debug)]
struct Configuration<'a> {
    verbose: bool,
    width: u32,
    height: u32,
    depth: u32,
    passes: u32,
    live: bool,
    output: &'a str,
    pixel_format: PixelFormat,
    integrator_backend: IntegratorBackend,
}

impl<'a> Configuration<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        verbose: bool,
        width: u32,
        height: u32,
        depth: u32,
        passes: u32,
        live: bool,
        output: &'a str,
        pixel_format: PixelFormat,
        integrator_backend: IntegratorBackend,
    ) -> Self {
        Self {
            verbose,
            width,
            height,
            depth,
            passes,
            live,
            output,
            pixel_format,
            integrator_backend,
        }
    }

    fn start(&mut self) -> Result<(), String> {
        if self.verbose {
            println!("{:#?}", self);
        }

        let (scene, camera) = cornell_box::create(self.width, self.height);
        let sampler = Mutex::new(RandomSampler);

        let integrator: Arc<dyn Integrator> = match self.integrator_backend {
            IntegratorBackend::Debug => Arc::new(DebugNormals),
            IntegratorBackend::Whitted => Arc::new(Whitted::new(self.depth)),
            IntegratorBackend::Path => Arc::new(Path::new(0, self.depth)),
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
                PixelFormat::U8 => renderer
                    .get_image_u8()
                    .save(&self.output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
                PixelFormat::U16 => renderer
                    .get_image_u16()
                    .save(&self.output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
            };

            Ok(())
        }
    }
}

#[derive(Debug)]
enum PixelFormat {
    U8,
    U16,
}

impl TryInto<PixelFormat> for &str {
    type Error = String;

    fn try_into(self) -> Result<PixelFormat, Self::Error> {
        match self {
            "u8" | "U8" => Ok(PixelFormat::U8),
            "u16" | "U16" => Ok(PixelFormat::U16),
            _ => Err(self.to_string()),
        }
    }
}

#[derive(Debug)]
enum IntegratorBackend {
    Debug, Whitted, Path,
}

impl TryInto<IntegratorBackend> for &str {
    type Error = String;

    fn try_into(self) -> Result<IntegratorBackend, Self::Error> {
        match self {
            "debug" | "Debug" | "DEBUG" => Ok(IntegratorBackend::Debug),
            "whitted" | "Whitted" | "WHITTED" => Ok(IntegratorBackend::Whitted),
            "path" | "Path" | "PATH" => Ok(IntegratorBackend::Path),
            _ => Err(self.to_string()),
        }
    }
}
