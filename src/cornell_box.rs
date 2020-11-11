use crate::color::Color;
use crate::geometry::aabb::Aabb;
use crate::geometry::capsule::Capsule;
use crate::geometry::sphere::Sphere;
use crate::geometry::tube::Tube;
use crate::render::bxdf::bsdf::BSDF;
use crate::render::bxdf::lambertian::LambertianReflection;
use crate::render::camera::Camera;
use crate::render::light::PointLight;
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

pub fn create(width: u32, height: u32) -> (Scene, Camera) {
    (create_box(), create_camera(width, height))
}

pub fn create_box() -> Scene {
    let mut scene = Scene::default();

    // walls
    scene.push_obj(left_wall());
    scene.push_obj(right_wall());
    scene.push_obj(back_wall());
    scene.push_obj(floor());
    scene.push_obj(ceiling());

    // objects
    scene.push_obj(sphere());
    scene.push_obj(capsule());
    scene.push_obj(tube());

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

fn light() -> PointLight {
    let point = Vec3::new(
        (LEFT_WALL + RIGHT_WALL) / 2.0,
        CEILING - (CEILING - FLOOR) / 20.0,
        (FRONT + BACK_WALL) / 2.0,
    );

    let color = Spectrum::white();

    PointLight::new(point, color)
}

fn sphere() -> SceneObject {
    // center on ground
    let center = Vec3::new(
        LEFT_WALL + RIGHT_WALL * 1.5,
        FLOOR + RADIUS,
        (FRONT + BACK_WALL) / 4.0,
    );

    let sphere = Sphere::new(center, RADIUS);

    let color = Spectrum::blue();
    let bxdf = LambertianReflection::new(color);
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(sphere), bsdf)
}

fn capsule() -> SceneObject {
    let from = Vec3::new(
        LEFT_WALL * 1.5 + RIGHT_WALL,
        FLOOR + RADIUS,
        (FRONT + BACK_WALL) / 1.5,
    );

    let height = (CEILING - FLOOR) / 4.0;
    let height = height.min(FLOOR + RADIUS + 1.0);

    let to = from + Vec3::unit_y() * height;

    let capsule = Capsule::new(from, to, RADIUS);

    let color = Spectrum::white() * 0.75;
    let bxdf = LambertianReflection::new(color);
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(capsule), bsdf)
}

fn tube() -> SceneObject {
    let radius = RADIUS / 4.0;
    let points = [
        Vec3::unit_y() * radius - Vec3::unit_x() * 2.0 - Vec3::unit_z() * 3.0,
        Vec3::unit_y() * radius - Vec3::unit_z() * 3.0,
        Vec3::unit_y() * radius - Vec3::unit_x() - Vec3::unit_z() * f32::sqrt(2.0),
        Vec3::unit_y() * radius - Vec3::unit_x() * 2.0 - Vec3::unit_z() * 3.0,
    ];

    let tube = Tube::new(&points, radius);
    let color = (Spectrum::red() + Spectrum::blue()) / 2.0;
    let bxdf = LambertianReflection::new(color);
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(tube), bsdf)
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
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(aabb), bsdf)
}

fn right_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(RIGHT_WALL, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::green();
    let bxdf = LambertianReflection::new(color);
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(aabb), bsdf)
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
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(aabb), bsdf)
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
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(aabb), bsdf)
}

fn ceiling() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, CEILING, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::white();
    let bxdf = LambertianReflection::new(color);
    let bsdf = BSDF::new(vec![Box::new(bxdf)]);

    SceneObject::new(Box::new(aabb), bsdf)
}
