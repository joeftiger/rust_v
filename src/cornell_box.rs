#![allow(dead_code)]
#![allow(unused_imports)]

use crate::render::bxdf::bsdf::BSDF;
use crate::render::bxdf::fresnel::{Dielectric, FresnelNoOp};
use crate::render::bxdf::oren_nayar::OrenNayar;
use crate::render::bxdf::specular::{SpecularReflection, SpecularTransmission};
use crate::render::camera::Camera;
use crate::render::light::{Light, PointLight};
use crate::render::material::Material;
use crate::render::scene::Scene;
use crate::render::scene_objects::SceneObject;
use crate::Spectrum;
use color::Color;
use geometry::aabb::Aabb;
use geometry::capsule::Capsule;
use geometry::cylinder::Cylinder;
use geometry::mesh::Mesh;
use geometry::sphere::Sphere;
use geometry::tube::Tube;
use std::sync::Arc;
use ultraviolet::{Bivec3, Rotor3, Vec3};

pub const LEFT_WALL: f32 = -3.0;
pub const RIGHT_WALL: f32 = 3.0;
pub const BACK_WALL: f32 = -6.0;
pub const FLOOR: f32 = 0.0;
pub const CEILING: f32 = 7.0;
pub const FRONT: f32 = 0.0;
pub const THICKNESS: f32 = 0.1;
pub const RADIUS: f32 = 1.0;
pub const FOVY: f32 = 70.0;

pub const SIGMA: f32 = 20.0;
pub const ANGLE: f32 = std::f32::consts::FRAC_PI_8;

pub const X_CENTER: f32 = (RIGHT_WALL + LEFT_WALL) / 2.0;
pub const Y_CENTER: f32 = (CEILING + FLOOR) / 2.0;
pub const Z_CENTER: f32 = (BACK_WALL + FRONT) / 2.0;

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
    // scene.push_obj(sphere());
    // scene.push_obj(capsule());
    // scene.push_obj(tube());
    // scene.push_obj(bunny());
    scene.push_obj(dragon());
    scene.push_obj(emitter());

    // light
    scene.push_light(light());

    scene.build_bvh();

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

fn light() -> Arc<dyn Light> {
    let point = Vec3::new(X_CENTER, Y_CENTER + (CEILING - Y_CENTER) * 0.5, Z_CENTER);

    let color = Spectrum::white() * 20.0;

    Arc::new(PointLight::new(point, color))
}

fn emitter() -> SceneObject {
    let center = Vec3::new(X_CENTER, CEILING + RADIUS * 2.0, Z_CENTER);

    let sphere = Sphere::new(center, RADIUS * 2.1);

    let color = Spectrum::white();
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::new(color * 10.0, bsdf);

    SceneObject::new(Box::new(sphere), material)
}

fn bunny() -> SceneObject {
    let file_name = "./resources/meshes/bunny.obj";
    let (model, _) = tobj::load_obj(file_name, true).expect("Could not load bunny file");
    let scale = Vec3::one() * 25.0;
    let center_floor = Vec3::new(X_CENTER, FLOOR, Z_CENTER);
    let rotation = Rotor3::default();

    let bunny = Mesh::load_scale_floor_rot((&model[0].mesh, scale, center_floor, rotation));

    let color = Spectrum::white();
    let dielectric = Arc::new(Dielectric::new(1.0, 1.3));
    let transmission = Box::new(SpecularTransmission::new(color, dielectric.clone()));
    let reflection = Box::new(SpecularReflection::new(color, dielectric));
    let bsdf = BSDF::new(vec![reflection, transmission]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(bunny), material)
}

fn dragon() -> SceneObject {
    let file_name = "./resources/meshes/dragon_4.obj";
    let (model, _) = tobj::load_obj(file_name, true).expect("Could not load dragon file");
    let scale = Vec3::one() * 25.0;
    let floor = Vec3::new(X_CENTER, FLOOR, Z_CENTER * 0.75);
    let rotation = Rotor3::from_rotation_xz(-ANGLE);

    let dragon = Mesh::load_scale_floor_rot((&model[0].mesh, scale, floor, rotation));

    let color = Spectrum::white();
    let dielectric = Arc::new(Dielectric::new(1.0, 1.3));
    let transmission = Box::new(SpecularTransmission::new(color, dielectric.clone()));
    let reflection = Box::new(SpecularReflection::new(color, dielectric));
    let bsdf = BSDF::new(vec![reflection, transmission]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(dragon), material)
}

fn sphere() -> SceneObject {
    // center on ground
    let center = Vec3::new(
        LEFT_WALL + RIGHT_WALL * 1.5,
        FLOOR + RADIUS,
        FRONT + (BACK_WALL - FRONT) * 0.75,
    );

    let sphere = Sphere::new(center, RADIUS);

    let color = Spectrum::white();
    let fresnel = Arc::new(Dielectric::new(1.0, 2.0));
    let spec_trans = SpecularTransmission::new(color, fresnel);
    let bsdf = BSDF::new(vec![Box::new(spec_trans)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(sphere), material)
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

    let color = Spectrum::white();
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(capsule), material)
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
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    // let alpha_x = roughness_to_alpha(0.5);
    // let alpha_y = roughness_to_alpha(0.5);
    // let distribution = BeckmannDistribution::new(alpha_x, alpha_y, true);
    // let microfacet =
    //     MicrofacetReflection::new(color, Box::new(distribution), Box::new(FresnelNoOp));
    // let bsdf = BSDF::new(vec![Box::new(microfacet)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(tube), material)
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
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(aabb), material)
}

fn right_wall() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(RIGHT_WALL, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::green();
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(aabb), material)
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
    // let spec_refl = SpecularReflection::new(color, Arc::new(FresnelNoOp));
    // let bsdf = BSDF::new(vec![Box::new(spec_refl)]);
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(aabb), material)
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
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(aabb), material)
}

fn ceiling() -> SceneObject {
    let aabb = Aabb::new(
        Vec3::new(LEFT_WALL - THICKNESS, CEILING, BACK_WALL - THICKNESS),
        Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
    );

    let color = Spectrum::white();
    let oren_nayar = OrenNayar::new(color, SIGMA);
    let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
    let material = Material::from(bsdf);

    SceneObject::new(Box::new(aabb), material)
}
