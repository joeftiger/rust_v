use crate::render::scene::Scene;
use crate::render::scene_objects::SceneObject;
use crate::geometry::aabb::Aabb;
use ultraviolet::Vec3;
use crate::color::Srgb;
use crate::render::bxdf::LambertianReflection;
use crate::render::light::Light;
use crate::render::camera::Camera;

pub const LEFT_WALL: f32 = -3.0;
pub const RIGHT_WALL: f32 = 3.0;
pub const BACK_WALL: f32 = -6.0;
pub const FLOOR: f32 = 0.0;
pub const CEILING: f32 = 7.0;
pub const FRONT: f32 = 0.0;
pub const THICKNESS: f32 = 0.001;

pub const FOVY: f32 = 75.0;

pub fn create_box() -> Scene {
    let mut scene = Scene::default();

    scene.push_obj(left_wall());
    scene.push_obj(right_wall());
    scene.push_obj(back_wall());
    scene.push_obj(floor());
    scene.push_obj(ceiling());

    scene.push_light(light());

    scene
}

pub fn create_camera(width: u32, height: u32) -> Camera {
    let position = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        (CEILING + FLOOR) / 2.0,
        FRONT + 5.0
    );

    let center = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        (CEILING + FLOOR) / 2.0,
        (FRONT + BACK_WALL) / 2.0
    );

    let up = Vec3::unit_y();

    Camera::new(position, center, up, FOVY, width, height)
}

fn left_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, FLOOR -THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(LEFT_WALL, CEILING + THICKNESS, FRONT)
    );

    let color = Srgb::red();
    let bxdf = LambertianReflection(color);


    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn right_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(RIGHT_WALL, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT)
    );

    let color = Srgb::green();
    let bxdf = LambertianReflection(color);


    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn back_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, BACK_WALL)
    );

    let color = Srgb::white();
    let bxdf = LambertianReflection(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn floor() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, FLOOR, FRONT)
    );

    let color = Srgb::white();
    let bxdf = LambertianReflection(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn ceiling() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, CEILING, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT)
    );

    let color = Srgb::white();
    let bxdf = LambertianReflection(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn light() -> Light {
    let point = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        CEILING - THICKNESS,
        (FRONT + BACK_WALL) / 2.0
    );

    let color = Srgb::white();

    Light::new(point, color)
}
