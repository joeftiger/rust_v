#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

use rust_v::cornell_box;
use rust_v::render::renderer::debug::NormalRenderer;
use rust_v::render::renderer::{Renderer, RgbRenderer};
use rust_v::render::window::RenderWindow;

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

        let renderer: Box<dyn Renderer> = {
            if demo.is_present(DEBUG_RENDERER) {
                Box::new(NormalRenderer::new(scene, camera))
            } else {
                Box::new(RgbRenderer::new(scene, camera))
            }
        };

        render(demo, renderer);
    }
}

#[cfg(not(feature = "live-window"))]
fn render(matches: &ArgMatches, mut renderer: Box<dyn Renderer>) {
    render_and_save(matches, renderer);
}

#[cfg(feature = "live-window")]
fn render(matches: &ArgMatches, renderer: Box<dyn Renderer>) {
    if matches.is_present(LIVE_WINDOW) {
        RenderWindow::new("Rust-V".to_string(), renderer)
            .expect("Can't create window")
            .start_rendering();
    } else {
        render_and_save(matches, renderer);
    }
}

fn render_and_save(matches: &ArgMatches, mut renderer: Box<dyn Renderer>) {
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
