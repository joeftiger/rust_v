use crate::color::Color;
use crate::geometry::aabb::Aabb;
use crate::geometry::sphere::Sphere;
use crate::render::bxdf::LambertianReflection;
use crate::render::camera::Camera;
use crate::render::light::Light;
use crate::render::scene::Scene;
use crate::render::scene_objects::SceneObject;
use crate::Spectrum;
use ultraviolet::Vec3;

pub const LEFT_WALL: f32 = -3.0;
pub const RIGHT_WALL: f32 = 3.0;
pub const BACK_WALL: f32 = -6.0;
pub const FLOOR: f32 = 0.0;
pub const CEILING: f32 = 7.0;
pub const FRONT: f32 = 0.0;
pub const THICKNESS: f32 = 0.001;

pub const RADIUS: f32 = 1.0;

pub const FOVY: f32 = 70.0;

pub fn create_box() -> Scene {
    let mut scene = Scene::default();

    // walls
    scene.push_obj(left_wall());
    scene.push_obj(right_wall());
    scene.push_obj(back_wall());
    scene.push_obj(floor());
    scene.push_obj(ceiling());

    // spheres
    scene.push_obj(sphere_0());

    // light
    scene.push_light(light());

    scene
}

pub fn create_camera(width: u32, height: u32) -> Camera {
    // center and a bit back
    let position = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        (CEILING + FLOOR) / 2.0,
        FRONT + 5.0,
    );

    // center
    let center = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        (CEILING + FLOOR) / 2.0,
        (FRONT + BACK_WALL) / 2.0,
    );

    let up = Vec3::unit_y();

    Camera::new(position, center, up, FOVY, width, height)
}

fn sphere_0() -> SceneObject {
    // center on ground
    let center = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        FLOOR + RADIUS,
        (FRONT + BACK_WALL) / 2.0,
    );

    let sphere = Sphere::new(center, RADIUS);

    let color = Spectrum::blue();
    let bxdf = LambertianReflection::new(color);

    SceneObject::new(Box::new(sphere), Box::new(bxdf))
}

fn left_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(
            LEFT_WALL - THICKNESS,
            FLOOR - THICKNESS,
            BACK_WALL - THICKNESS,
        ),
        Vec3::new(LEFT_WALL, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::red();
    let bxdf = LambertianReflection::new(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn right_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(RIGHT_WALL, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::green();
    let bxdf = LambertianReflection::new(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn back_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(
            LEFT_WALL - THICKNESS,
            FLOOR - THICKNESS,
            BACK_WALL - THICKNESS,
        ),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, BACK_WALL),
    );

    let color = Spectrum::white();
    let bxdf = LambertianReflection::new(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn floor() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(
            LEFT_WALL - THICKNESS,
            FLOOR - THICKNESS,
            BACK_WALL - THICKNESS,
        ),
        Vec3::new(RIGHT_WALL + THICKNESS, FLOOR, FRONT),
    );

    let color = Spectrum::white();
    let bxdf = LambertianReflection::new(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn ceiling() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, CEILING, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::white();
    let bxdf = LambertianReflection::new(color);

    SceneObject::new(Box::new(aabb), Box::new(bxdf))
}

fn light() -> Light {
    let point = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        CEILING - (CEILING - FLOOR) / 20.0,
        (FRONT + BACK_WALL) / 2.0,
    );

    let color = Spectrum::white();

    Light::new(point, color, 25.0)
}
