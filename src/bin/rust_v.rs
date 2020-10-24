use clap::{App, Arg};
use ultraviolet::Vec3;

use rust_v::color::Srgb;
use rust_v::geometry::sphere::Sphere;
use rust_v::render::camera::Camera;
use rust_v::render::renderer::debug::NormalRenderer;
use rust_v::render::renderer::Renderer;
use rust_v::render::scene::Scene;
use rust_v::render::scene_objects::SceneObject;
use rust_v::render::window::RenderWindow;

const LIVE_WINDOW: &str = "LIVE_WINDOW";

fn main() {
    let app = init_help();
    let matches = app.get_matches();

    let mut renderer = create_renderer();

    if matches.is_present(LIVE_WINDOW) {
        let mut window =
            RenderWindow::new("Rust-V".to_string(), renderer).expect("Can't create window");

        window.start_rendering();
    } else {
        renderer.render_all();

        let image = renderer.get_image();
        image.save("./rendering.png").unwrap();
    }
}

fn init_help<'a, 'b>() -> App<'a, 'b> {
    App::new("Rust-V")
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
}

fn create_renderer() -> impl Renderer {
    let mut objects = Vec::new();
    for _ in 0..20 {
        let x = fastrand::f32() * 10.0 - 5.0;
        let y = fastrand::f32() * 10.0 - 5.0;
        let z = fastrand::f32() * 10.0 - 5.0;
        let center = Vec3::new(x, y, z);
        let radius = fastrand::f32() * 2.0;
        let sphere = Sphere::new(center, radius);
        let color = Srgb::from(center);

        let object = SceneObject::new(sphere, color);

        objects.push(object);
    }

    let scene = Scene::new(objects);
    let camera = Camera::new(
        -7.0 * Vec3::unit_x(),
        Vec3::zero(),
        Vec3::unit_z(),
        90.0,
        1280,
        720,
    );

    NormalRenderer::new(scene, camera)
}
