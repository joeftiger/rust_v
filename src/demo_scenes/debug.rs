#![allow(dead_code)]

use crate::demo_scenes::{DemoScene, FOVY};
use crate::bxdf::bsdf::BSDF;
use crate::bxdf::lambertian::LambertianReflection;
use crate::render::camera::Camera;
use crate::render::light::{Light, PointLight};
use crate::render::material::Material;
use crate::render::scene::Scene;
use crate::render::scene_objects::Object;
use crate::Spectrum;
use color::Color;
use geometry::aabb::Aabb;
use geometry::sphere::Sphere;
use std::sync::Arc;
use ultraviolet::{Rotor3, Vec3};
use geometry::Geometry;

pub struct DebugScene;

impl DebugScene {
    fn plane() -> Object<Aabb> {
        let min = Vec3::new(-10000.0, -5.0, -10000.0);
        let max = Vec3::new(10000.0, 0.0, 10000.0);
        let aabb =Aabb::new(min, max);

        let lambertian = LambertianReflection::new(Spectrum::white());
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);
        let material = Material::from(bsdf);

        Object::new(aabb, material)
    }

    fn sphere() -> Object<Sphere> {
        let sphere =Sphere::new(Vec3::unit_y(), 1.0);

        let lambertian = LambertianReflection::new(Spectrum::white());
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);
        let material = Material::new(
            // None,
            Some(Spectrum::white()),
            bsdf,
        );

        Object::new(sphere, material)
    }

    fn top_light() -> Arc<dyn Light> {
        Arc::new(PointLight::new(
            Vec3::new(0.0, 5.0, 0.0),
            Spectrum::white() * 10.0,
        ))
    }

    fn top_right_light() -> Arc<dyn Light> {
        Arc::new(PointLight::new(
            Vec3::new(0.0, 5.0, 0.0).rotated_by(Rotor3::from_rotation_xy(30.0)),
            Spectrum::white() * 10.0,
        ))
    }

    fn top_left_light() -> Arc<dyn Light> {
        Arc::new(PointLight::new(
            Vec3::new(0.0, 5.0, 0.0).rotated_by(Rotor3::from_rotation_xy(-30.0)),
            Spectrum::white() * 10.0,
        ))
    }

    fn create_camera(width: u32, height: u32) -> Camera {
        Camera::new(
            Vec3::new(0.0, 2.0, -4.0),
            -Vec3::unit_y() / 4.0,
            Vec3::unit_y(),
            FOVY,
            width,
            height,
        )
    }
}

impl DemoScene for DebugScene {
    fn create(width: u32, height: u32) -> (Scene, Camera) {
        let mut scene = Scene::default();

        scene.push_obj(Arc::new(Self::plane()));
        scene.push_obj(Arc::new(Self::sphere()));

        // scene.push_light(Self::top_left_light());
        // scene.push_light(Self::top_light());
        // scene.push_light(Self::top_right_light());

        scene.build_bvh();

        (scene, Self::create_camera(width, height))
    }
}
