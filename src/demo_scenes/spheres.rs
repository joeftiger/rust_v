use crate::bxdf::bsdf::BSDF;
use crate::bxdf::fresnel::{Dielectric, FresnelNoOp};
use crate::bxdf::lambertian::LambertianReflection;
use crate::bxdf::oren_nayar::OrenNayar;
use crate::bxdf::specular::{SpecularReflection, SpecularTransmission};
use crate::demo_scenes::{DemoScene, FOVY, SIGMA};
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

const RADIUS: f32 = 0.5;

pub struct SphereScene;

impl SphereScene {
    fn ground() -> Instance {
        let min = Vec3::new(-10000.0, -5.0, -10000.0);
        let max = Vec3::new(10000.0, 0.0, 10000.0);
        let aabb = Aabb::new(min, max);

        let lambertian = OrenNayar::new(Spectrum::white(), SIGMA);
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }

    fn random_pos() -> Vec3 {
        let x = fastrand::f32() * 10.0 - 5.0;
        let z = fastrand::f32() * 10.0 - 5.0;

        Vec3::new(x, RADIUS + 0.05, z)
    }

    fn random_color() -> Spectrum {
        let rand = fastrand::f32() * 1.5;

        if rand < 0.25 {
            Spectrum::red()
        } else if rand < 0.5 {
            Spectrum::green()
        } else if rand < 0.75 {
            Spectrum::blue()
        } else {
            Spectrum::white()
        }
    }

    fn random_bsdf(color: Spectrum) -> (bool, BSDF) {
        let rand = fastrand::f32();

        if color == Spectrum::white() {
            if rand < 0.6 {
                let oren_nayar = LambertianReflection::new(color);
                (true, BSDF::new(vec![Box::new(oren_nayar)]))
            } else {
                let oren_nayar = OrenNayar::new(Spectrum::black(), SIGMA);
                (false, BSDF::new(vec![Box::new(oren_nayar)]))
            }
        } else {
            let oren_nayar = OrenNayar::new(color, SIGMA);
            (false, BSDF::new(vec![Box::new(oren_nayar)]))
        }
    }

    fn emitter() -> Instance {
        let center = Vec3::new(0.0, 50.0, 0.0);
        let sphere = Sphere::new(center, 0.05);

        let bsdf = BSDF::empty();

        Emitter(Arc::new(EmitterObj::new(
            sphere,
            Arc::new(bsdf),
            (Spectrum::red() + Spectrum::green()) * 1000.0,
        )))
    }

    fn create_scene() -> Scene {
        let mut scene = Scene::default();

        for _ in 0..5 {
            for _ in 0..5 {
                let center = Self::random_pos();
                let sphere = Sphere::new(center, RADIUS);

                let color = Self::random_color();
                let bsdf = Self::random_bsdf(color);

                let obj = if bsdf.0 {
                    Emitter(Arc::new(EmitterObj::new(
                        sphere,
                        Arc::new(bsdf.1),
                        color * 5.0,
                    )))
                } else {
                    Receiver(Arc::new(ReceiverObj::new(sphere, Arc::new(bsdf.1))))
                };

                scene.add(obj);
            }
        }

        scene.add(Self::ground())
            .add(Self::emitter());

        scene.build_bvh();
        scene
    }

    fn create_camera(width: u32, height: u32) -> Camera {
        let position = Vec3::new(0.0, 10.0, 1.0);

        let center = Vec3::new(0.0, 2.0, 0.0);

        let up = Vec3::unit_y();

        Camera::new(position, center, up, FOVY, width, height)
    }
}

impl DemoScene for SphereScene {
    fn create(width: u32, height: u32) -> (Scene, Camera) {
        fastrand::seed(0);
        (Self::create_scene(), Self::create_camera(width, height))
    }
}
