#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

use rust_v::cornell_box;
use rust_v::render::integrator::debug_normals::DebugNormals;
use rust_v::render::integrator::whitted::Whitted;
use rust_v::render::integrator::Integrator;
use rust_v::render::renderer::Renderer;
use rust_v::render::sampler::RandomSampler;

#[cfg(feature = "live-window")]
use rust_v::render::window::RenderWindow;

#[cfg(feature = "live-window")]
const LIVE_WINDOW: &str = "live_window";
const DEMO: &str = "demo";
const DEBUG_RENDERER: &str = "DEBUG";
#[allow(dead_code)]
const INPUT: &str = "INPUT";
const OUTPUT: &str = "OUTPUT";

fn main() {
    #[cfg(not(feature = "live-window"))]
    let yaml = load_yaml!("cli.yml");
    #[cfg(feature = "live-window")]
    let yaml = load_yaml!("cli-live-window.yml");

    let matches = App::from_yaml(yaml).get_matches();

    if let Some(demo) = matches.subcommand_matches(DEMO) {
        let (scene, camera) = cornell_box::create(900, 900);

        let sampler = Box::new(RandomSampler);

        let integrator: Box<dyn Integrator> = {
            if demo.is_present(DEBUG_RENDERER) {
                Box::new(DebugNormals)
            } else {
                Box::new(Whitted::new(5))
            }
        };

        let renderer = Renderer::new(scene, camera, sampler, integrator);

        render(demo, renderer);
    }
}

#[cfg(not(feature = "live-window"))]
fn render(matches: &ArgMatches, renderer: Renderer) {
    render_and_save(matches, renderer);
}

#[cfg(feature = "live-window")]
fn render(matches: &ArgMatches, renderer: Renderer) {
    if matches.is_present(LIVE_WINDOW) {
        RenderWindow::new("Rust-V".to_string(), renderer)
            .expect("Can't create window")
            .start_rendering();
    } else {
        render_and_save(matches, renderer);
    }
}

fn render_and_save(matches: &ArgMatches, mut renderer: Renderer) {
    renderer.render_all();
    let image = renderer.get_image_u16();

    if let Some(output) = matches.value_of_os(OUTPUT) {
        image
            .save(&output)
            .unwrap_or_else(|_| panic!("Could not save to: {:?}", output));
    } else {
        let start = std::time::SystemTime::now();
        let since = start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        let file = format!("{}.png", since.as_nanos());

        image
            .save(&file)
            .unwrap_or_else(|_| panic!("Could not save to: {}", file));
    }
}
