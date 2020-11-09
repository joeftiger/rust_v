use clap::{App, Arg, SubCommand};

use rust_v::cornell_box;
use rust_v::render::camera::Camera;
use rust_v::render::renderer::debug::NormalRenderer;
use rust_v::render::renderer::{Renderer, RgbRenderer};
use rust_v::render::scene::Scene;
use rust_v::render::window::RenderWindow;

const LIVE_WINDOW: &str = "LIVE_WINDOW";
const DEMO: &str = "demo";
const DEBUG_RENDERER: &str = "DEBUG_RENDERER";
const INPUT: &str = "IN";
const OUTPUT: &str = "OUT";

fn main() {
    let mut app = init_help();
    let matches = app.clone().get_matches();

    if let Some(demo) = matches.subcommand_matches(DEMO) {
        let (scene, camera) = create_cornell_box();

        let mut renderer: Box<dyn Renderer>;
        if demo.is_present(DEBUG_RENDERER) {
            renderer = Box::new(NormalRenderer::new(scene, camera));
        } else {
            renderer = Box::new(RgbRenderer::new(scene, camera));
        }

        if demo.is_present(LIVE_WINDOW) {
            let mut window =
                RenderWindow::new("Rust-V".to_string(), renderer).expect("Can't create window");

            window.start_rendering();
        } else {
            renderer.render_all();

            let image = renderer.get_image_u16();
            image.save("./rendering.png").unwrap();
        }
    } else {
        app.print_help().expect("Could not print help message");
    }
}

fn init_help<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("Rust-V")
        .version("0.0.1")
        .author("Julius Oeftiger")
        .about("A rust ray tracer supporting rgb and spectral ray tracing")
        .arg(
            Arg::with_name(LIVE_WINDOW)
                .short("l")
                .long("live")
                .help("Use a live window to show rendering progressively. The window allows saving with [Ctrl + S]")
                .required(false)
        )
        .arg(
            Arg::with_name(INPUT)
                .required(false)
        )
        .arg(
            Arg::with_name(OUTPUT)
                .required(false)
        );

    let demo = SubCommand::with_name(DEMO)
        .version("0.0.1")
        .author("Julius Oeftiger")
        .about("The cornell box demo scene")
        .arg(
            Arg::with_name(DEBUG_RENDERER)
                .long("--debug")
                .help("Use a debugging renderer to render surface normals.")
                .required(false),
        )
        .arg(
            Arg::with_name(LIVE_WINDOW)
                .long("live")
                .help("Use a live window to show rendering progressively. The window allows saving with [Ctrl + S]")
                .required(false)
        );

    app.subcommand(demo)
}

fn create_cornell_box() -> (Scene, Camera) {
    (
        cornell_box::create_box(),
        cornell_box::create_camera(900, 900),
    )
}
