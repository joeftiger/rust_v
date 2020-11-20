#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

use rust_v::cornell_box;
use rust_v::render::integrator::debug_normals::DebugNormals;
use rust_v::render::integrator::Integrator;
use rust_v::render::integrator::whitted::Whitted;
use rust_v::render::renderer::Renderer;
use rust_v::render::sampler::RandomSampler;
#[cfg(feature = "live-window")]
use rust_v::render::window::RenderWindow;

#[cfg(feature = "live-window")]
const LIVE_WINDOW: &str = "LIVE-WINDOW";
const DEMO: &str = "demo";
const DEBUG_RENDERER: &str = "DEBUG";
#[allow(dead_code)]
const INPUT: &str = "INPUT";
const OUTPUT: &str = "OUTPUT";
const PASSES: &str = "PASSES";
const DEPTH: &str = "DEPTH";

fn main() {
    #[cfg(not(feature = "live-window"))]
        let yaml = load_yaml!("cli.yml");
    #[cfg(feature = "live-window")]
        let yaml = load_yaml!("cli-live-window.yml");

    let matches = App::from_yaml(yaml).get_matches();

    if let Some(demo) = matches.subcommand_matches(DEMO) {
        let (scene, camera) = cornell_box::create(2000, 2000);

        let sampler = Box::new(RandomSampler);

        let integrator: Box<dyn Integrator> = {
            if demo.is_present(DEBUG_RENDERER) {
                Box::new(DebugNormals)
            } else {
                let depth = match demo.value_of(DEPTH).unwrap_or("6").parse() {
                    Err(msg) => panic!("Unable to pass DEPTH: {}", msg),
                    Ok(val) => val
                };

                Box::new(Whitted::new(depth))
            }
        };

        let renderer = Renderer::new(scene, camera, sampler, integrator);

        if let Err(msg) = render(demo, renderer) {
            panic!("Error: {}", msg)
        }
    }
}

#[cfg(not(feature = "live-window"))]
fn render(matches: &ArgMatches, renderer: Renderer) -> Result<(), String> {
    render_and_save(matches, renderer)
}

#[cfg(feature = "live-window")]
fn render(matches: &ArgMatches, renderer: Renderer) -> Result<(), String> {
    if matches.is_present(LIVE_WINDOW) {
        RenderWindow::new("Rust-V".to_string(), renderer)
            .map_err(|e| format!("Unable to create window: {}", e))?
            .start_rendering();
        Ok(())
    } else {
        render_and_save(matches, renderer)
    }
}

fn render_and_save(matches: &ArgMatches, mut renderer: Renderer) -> Result<(), String> {
    let passes = matches.value_of(PASSES)
        .unwrap_or("1")
        .parse::<u32>()
        .map_err(|e| format!("Unable to pass PASSES: {}", e))?;


    renderer.render_all(passes);
    let image = renderer.get_image_u16();

    if let Some(output) = matches.value_of_os(OUTPUT) {
        image.save(&output).map_err(|e| format!("Unable to save image: {}", e))
    } else {
        let since = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?;
        let file = format!("{}.png", since.as_nanos());

        image.save(&file).map_err(|e| format!("Unable to save image: {}", e))
    }
}
