use rust_v::geometry::sphere::Sphere;
use ultraviolet::Vec3;
use rust_v::render::camera::Camera;
use rust_v::render::scene::Scene;
use rust_v::render::renderer::{RgbRenderer, Renderer};
use rust_v::render::scene_objects::SceneObject;
use rust_v::color::srgb::Srgb;
use rust_v::render::window::RenderWindow;

fn main() {
    let objects = vec![
        SceneObject::new(Sphere::default(), Srgb::new(1.0, 0.0, 0.0)),
    ];

    let scene = Scene::new(objects);

    let camera = Camera::new(
        -2.0 * Vec3::unit_x(),
        Vec3::zero(),
        Vec3::unit_z(),
        120.0,
        1280,
        720
    );

    let mut renderer = RgbRenderer::new(scene, camera);

    while !renderer.is_done() {
        renderer.render_pass();
    }

    let image = renderer.get_image();
    image.save("./rendering.png").unwrap();

    // let mut window = RenderWindow::new("Rust-V".to_string(), renderer).unwrap();
    //
    // window.start_rendering();
}
