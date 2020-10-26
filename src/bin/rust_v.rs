use clap::{App, Arg, SubCommand};
use ultraviolet::Vec3;

use rust_v::color::Srgb;
use rust_v::geometry::aabb::Aabb;
use rust_v::geometry::point::Point;
use rust_v::geometry::sphere::Sphere;
use rust_v::render::camera::Camera;
use rust_v::render::light::Light;
use rust_v::render::renderer::debug::NormalRenderer;
use rust_v::render::renderer::{Renderer, RgbRenderer};
use rust_v::render::scene::Scene;
use rust_v::render::scene_objects::SceneObject;
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
        let (scene, camera) = create_random_scene_camera();

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

            let image = renderer.get_image();
            image.save("./rendering.png").unwrap();
        }
    } else {
        app.print_help().expect("Could not print help message");
        return;
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
        .about("A randomized demo scene")
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

fn create_random_scene_camera() -> (Scene, Camera) {
    let width = 1280.0;
    let height = 720.0;
    let fovy = 45.0;
    let num = 30;

    println!("Creating {} random objects...", num);
    let mut objects = Vec::new();
    for _ in 0..num {
        let x = fastrand::f32() * width - width / 2.0;
        let y = fastrand::f32() * width - width / 2.0;
        let z = fastrand::f32() * height - height / 2.0;
        let center = Vec3::new(x, y, z);
        let color = Srgb::from(center.normalized());

        let object;
        if fastrand::f32() < 0.5 {
            let radius = fastrand::f32() * width * 4.0 / num as f32;
            let sphere = Sphere::new(center, radius);

            object = SceneObject::new(Box::new(sphere), color);
        } else {
            let size = fastrand::f32() * height / 5.0;
            let offset = Vec3::one() * size;
            let aabb = Aabb::new(center - offset, center + offset);

            object = SceneObject::new(Box::new(aabb), color);
        }

        objects.push(object);
    }

    println!("Creating lights...");
    let lights = vec![Light::new(
        Vec3::zero(),
        Box::new(Point::new(Vec3::zero())),
        Srgb::from(Vec3::one()),
        width * height,
    )];

    let scene = Scene::new(objects, lights);
    let camera = Camera::new(
        -width * Vec3::unit_x(),
        Vec3::zero(),
        Vec3::unit_z(),
        fovy,
        width as u32,
        height as u32,
    );

    (scene, camera)
}
