#![allow(dead_code)]

use crate::bxdf::bsdf::BSDF;
use crate::bxdf::lambertian::LambertianReflection;
use crate::demo_scenes::{DemoScene, FOVY};
use crate::render::camera::Camera;
use crate::render::objects::emitter::EmitterObj;
use crate::render::objects::receiver::ReceiverObj;
use crate::render::objects::Instance;
use crate::render::objects::Instance::{Emitter, Receiver};
use crate::render::scene::Scene;
use crate::Spectrum;
use color::Color;
use geometry::aabb::Aabb;
use geometry::sphere::Sphere;
use std::sync::Arc;
use ultraviolet::Vec3;

pub struct DebugScene;

impl DebugScene {
    fn plane() -> Instance {
        let min = Vec3::new(-10000.0, -5.0, -10000.0);
        let max = Vec3::new(10000.0, 0.0, 10000.0);
        let aabb = Aabb::new(min, max);

        let lambertian = LambertianReflection::new(Spectrum::white());
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }

    fn sphere() -> Instance {
        let sphere = Sphere::new(Vec3::unit_y(), 1.0);

        let lambertian = LambertianReflection::new(Spectrum::black());
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);

        Emitter(Arc::new(EmitterObj::new(
            sphere,
            Arc::new(bsdf),
            Spectrum::white() * 3.0,
        )))
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

        scene.add(Self::plane()).add(Self::sphere());

        scene.build_bvh();

        (scene, Self::create_camera(width, height))
    }
}
