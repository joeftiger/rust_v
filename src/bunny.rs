use crate::render::scene::Scene;
use crate::render::camera::Camera;
use crate::render::scene_objects::SceneObject;
use std::sync::Arc;
use crate::render::light::{Light, PointLight};
use ultraviolet::Vec3;
use crate::Spectrum;
use geometry::mesh::Mesh;
use color::Color;
use crate::render::bxdf::oren_nayar::OrenNayar;
use crate::render::bxdf::bsdf::BSDF;
use geometry::aabb::Aabb;
use crate::render::bxdf::fresnel::Dielectric;
use crate::render::bxdf::specular::{SpecularTransmission, SpecularReflection};
use crate::render::bxdf::lambertian::LambertianReflection;

pub const FOVY: f32 = 70.0;
pub const SIGMA: f32 = 20.0;

pub fn create(width: u32, height: u32) -> (Scene, Camera) {
    (create_box(), create_camera(width, height))
}

pub fn create_box() -> Scene {
    let mut scene = Scene::default();

    // environment
    scene.push_obj(floor());

    // objects
    scene.push_obj(bunny());

    // light
    scene.push_light(light());

    scene
}

pub fn create_camera(width: u32, height: u32) -> Camera {
    let position = Vec3::new(
        0.0,
        1.0,
        1.0,
    );
    let position = position;

    let center = -Vec3::unit_z();

    let up = Vec3::unit_y();

    Camera::new(position, center, up, FOVY, width, height)
}

fn light() -> Arc<dyn Light> {
    let point = Vec3::new(
        0.0,
        1.0,
        1.0,
    );

    let color = Spectrum::new_const(20.0);

    Arc::new(PointLight::new(point, color))
}

fn bunny() -> SceneObject {
    let file_name = "./resources/meshes/bunny.obj";
    let (model, _) = tobj::load_obj(file_name, true)
        .expect("Could not load bunny file");
    let scale = Vec3::one() * 3.0;

    let bunny = Mesh::from((&model[0].mesh, scale));

    let color = Spectrum::red();
    let dielectric = Arc::new(
        Dielectric::new(1.0, 1.3)
    );
    // let transmission = Box::new(
    //     SpecularTransmission::new(color, dielectric.clone())
    // );
    // let reflection = Box::new(
    //     SpecularReflection::new(color, dielectric)
    // );
    let lambertian = Box::new(
        LambertianReflection::new(color)
    );
    // let oren_nayar = Box::new(OrenNayar::new(color, SIGMA));
    let bsdf = BSDF::new(vec![
        // oren_nayar,
        // reflection,
        lambertian,
        // transmission,
    ]);


    SceneObject::new(Box::new(bunny), bsdf)
}

fn floor() -> SceneObject {
    let min = Vec3::new(-100.0, -1.0, -100.0);
    let max = Vec3::new(100.0, 0.0, 100.0);
    let aabb = Aabb::new(min, max);

    let color = Spectrum::white();
    let oren_nayar = Box::new(OrenNayar::new(color, SIGMA));
    let bsdf = BSDF::new(vec![oren_nayar]);

    SceneObject::new(Box::new(aabb), bsdf)
}
