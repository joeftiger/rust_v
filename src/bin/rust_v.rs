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

fn main() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml).get_matches();

    if let Some(demo) = matches.subcommand_matches(DEMO) {
        let verbose = demo.is_present(VERBOSE);
        let debug = demo.is_present(DEBUG);
        let width = demo.value_of(WIDTH).unwrap_or("900").parse().unwrap();
        let height = demo.value_of(HEIGHT).unwrap_or("900").parse().unwrap();
        let depth = demo.value_of(DEPTH).unwrap_or("6").parse().unwrap();
        let passes = demo.value_of(PASSES).unwrap_or("1").parse().unwrap();
        let live = demo.is_present(LIVE);

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

        let mut main = Main::new(verbose, debug, width, height, depth, passes, live, output);
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
        }
    }

    fn start(&mut self) -> Result<(), String> {
        if self.verbose {
            println!("{:#?}", self);
        }

        let (scene, camera) = cornell_box::create(self.width, self.height);
        let sampler = Box::new(RandomSampler);
        let integrator: Box<dyn Integrator> = if self.debug {
            Box::new(DebugNormals)
        } else {
            Box::new(Whitted::new(self.depth))
        };
        let mut renderer = Renderer::new(scene, camera, sampler, integrator);

        if self.live {
            RenderWindow::new("Rust-V".to_string(), &mut renderer)
                .map_err(|e| format!("Unable to create window: {}", e))?
                .start_rendering();
            Ok(())
        } else {
            let bar = ProgressBar::new(renderer.len_pixels() as u64);
            bar.set_style(ProgressStyle::default_bar().template(
                "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining]",
            ));

            renderer.render_all_with(self.passes, &bar);
            bar.finish();

            let image = renderer.get_image_u16();

            image
                .save(&self.output)
                .map_err(|e| format!("Unable to save image: {}", e))
        }
    }
}
