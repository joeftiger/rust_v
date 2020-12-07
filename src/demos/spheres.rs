use crate::demos::{DemoScene, FOVY, SIGMA};
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
use geometry::sphere::Sphere;
use std::sync::Arc;
use ultraviolet::Vec3;
use geometry::aabb::Aabb;
use crate::render::bxdf::lambertian::LambertianReflection;

const RADIUS: f32 = 0.5;

pub struct Spheres;

impl Spheres {
    fn ground() -> SceneObject {
        let min = Vec3::new(-10000.0, -5.0, -10000.0);
        let max = Vec3::new(10000.0, 0.0, 10000.0);
        let aabb = Box::new(Aabb::new(min, max));

        let lambertian = LambertianReflection::new(Spectrum::white());
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);
        let material = Material::from(bsdf);

        SceneObject::new(aabb, material)
    }

    fn random_pos() -> Vec3 {
        let x = fastrand::f32() * 20.0 - 10.0;
        let z = fastrand::f32() * 20.0 - 10.0;

        Vec3::new(x, RADIUS, z)
    }

    fn random_color() -> Spectrum {
        let rand = fastrand::f32() * 1.5;

        if rand < 0.2 {
            Spectrum::red()
        } else if rand < 0.4 {
            Spectrum::green()
        } else if rand < 0.6 {
            Spectrum::blue()
        } else if rand < 0.8 {
            Spectrum::new_const(rand)
        } else {
            Spectrum::white()
        }
    }

    fn random_material(color: Spectrum) -> Material {
        let rand = fastrand::f32();

        if color == Spectrum::white() {
            if rand < 0.6 {
                let oren_nayar = OrenNayar::new(color, SIGMA);
                let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
                Material::new(color * 1000.0, bsdf)
            } else if rand < 0.8 {
                let reflection = SpecularReflection::new(color, Arc::new(FresnelNoOp));
                let bsdf = BSDF::new(vec![Box::new(reflection)]);
                Material::from(bsdf)
            } else {
                let fresnel = Arc::new(Dielectric::new(1.0, 1.5));
                let transmission = SpecularTransmission::new(color, fresnel);
                let bsdf = BSDF::new(vec![Box::new(transmission)]);
                Material::from(bsdf)
            }
        } else {
            let oren_nayar = OrenNayar::new(color, SIGMA);
            let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);
            Material::from(bsdf)
        }
    }

    fn big_emitter() -> SceneObject {
        let min = Vec3::new(-100.0, 100.0, -100.0);
        let max = Vec3::new(100.0, 200.0, 100.0);
        let aabb = Box::new(Aabb::new(min, max));


        let lambertian = LambertianReflection::new(Spectrum::black());
        let bsdf = BSDF::new(vec![Box::new(lambertian)]);
        let material = Material::new(Spectrum::white() * 10.0, bsdf);

        SceneObject::new(aabb, material)
    }

    fn light() -> Arc<dyn Light> {
        let point = Vec3::new(0.0, 90.0, 0.0);
        let color = Spectrum::white() * 10000.0;

        Arc::new(PointLight::new(point, color))
    }

    fn create_scene() -> Scene {
        let mut scene = Scene::default();

        for _ in 0..10 {
            for _ in 0..10 {
                let center = Self::random_pos();
                let sphere = Box::new(Sphere::new(center, RADIUS));

                let color = Self::random_color();
                let material = Self::random_material(color);

                let object = SceneObject::new(sphere, material);
                scene.push_obj(object);
            }
        }

        scene.push_obj(Self::ground());
        scene.push_obj(Self::big_emitter());
//        scene.push_light(Self::light());

        scene.build_bvh();
        scene
    }

    fn create_camera(width: u32, height: u32) -> Camera {
        let position = Vec3::new(0.0, 5.0, -10.0);

        let center = Vec3::new(0.0, 2.0, 0.0);

        let up = Vec3::unit_y();

        Camera::new(position, center, up, FOVY, width, height)
    }
}

impl DemoScene for Spheres {
    fn create(width: u32, height: u32) -> (Scene, Camera) {
        fastrand::seed(0);
        (Self::create_scene(), Self::create_camera(width, height))
    }
}
