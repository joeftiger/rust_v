#![allow(dead_code)]
#![allow(unused_imports)]

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
use geometry::capsule::Capsule;
use geometry::cylinder::Cylinder;
use geometry::mesh::Mesh;
use geometry::point::Point;
use geometry::sphere::Sphere;
use geometry::tube::Tube;
use geometry::Geometry;
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

pub const ANGLE: f32 = std::f32::consts::FRAC_PI_8;

pub const X_CENTER: f32 = (RIGHT_WALL + LEFT_WALL) / 2.0;
pub const Y_CENTER: f32 = (CEILING + FLOOR) / 2.0;
pub const Z_CENTER: f32 = (BACK_WALL + FRONT) / 2.0;

pub struct CornellScene;

impl CornellScene {
    fn create_box() -> Scene {
        let mut scene = Scene::default();

        // walls
        scene
            .add(Self::left_wall())
            .add(Self::right_wall())
            .add(Self::back_wall())
            .add(Self::floor())
            .add(Self::ceiling())
            // objects
            .add(Self::sphere())
            .add(Self::capsule())
            .add(Self::tube())
            // lights
            .add(Self::emitter());

        scene.build_bvh();

        scene
    }

    fn create_camera(width: u32, height: u32) -> Camera {
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

    fn emitter() -> Instance {
        let position = Vec3::new(X_CENTER, CEILING - RADIUS, Z_CENTER);
        let point = Point::new(position);

        // let center = Vec3::new(X_CENTER, CEILING, Z_CENTER);
        // let sphere = Sphere::new(center, RADIUS);

        let color = Spectrum::white();
        let oren_nayar = OrenNayar::new(color, SIGMA);
        let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);

        Emitter(Arc::new(EmitterObj::new(
            point,
            Arc::new(bsdf),
            color * 2.0,
        )))
    }

    fn bunny() -> Instance {
        let file_name = "./resources/meshes/bunny.obj";
        let (model, _) = tobj::load_obj(file_name, true).expect("Could not load bunny file");
        let scale = Vec3::one() * 25.0;
        let center_floor = Vec3::new(X_CENTER, FLOOR, Z_CENTER);
        let rotation = Rotor3::default();

        let bunny = Mesh::load_scale_floor_rot((&model[0].mesh, scale, center_floor, rotation));

        let color = Spectrum::white();
        let dielectric = Arc::new(Dielectric::new(1.0, 1.3));
        let transmission = Box::new(SpecularTransmission::new(color, dielectric));
        // let reflection = Box::new(SpecularReflection::new(color, dielectric));
        let bsdf = BSDF::new(vec![transmission]);

        Receiver(Arc::new(ReceiverObj::new(bunny, Arc::new(bsdf))))
    }

    fn dragon() -> Instance {
        let file_name = "./resources/meshes/dragon_4.obj";
        let (model, _) = tobj::load_obj(file_name, true).expect("Could not load dragon file");
        let scale = Vec3::one() * 25.0;
        let floor = Vec3::new(X_CENTER, FLOOR, Z_CENTER * 0.75);
        let rotation = Rotor3::from_rotation_xz(-ANGLE);

        let dragon = Mesh::load_scale_floor_rot((&model[0].mesh, scale, floor, rotation));

        let color = Spectrum::white();
        let dielectric = Arc::new(Dielectric::new(1.0, 1.3));
        let transmission = Box::new(SpecularTransmission::new(color, dielectric.clone()));
        let reflection = Box::new(SpecularReflection::new(color * 0.25, dielectric));
        let bsdf = BSDF::new(vec![reflection, transmission]);

        Receiver(Arc::new(ReceiverObj::new(dragon, Arc::new(bsdf))))
    }

    fn sphere() -> Instance {
        // center on ground
        let center = Vec3::new(
            LEFT_WALL + RIGHT_WALL * 1.5,
            FLOOR + RADIUS,
            FRONT + (BACK_WALL - FRONT) * 0.75,
        );

        let sphere = Sphere::new(center, RADIUS);

        let color = Spectrum::white();
        let fresnel = Arc::new(Dielectric::new(1.0, 1.1));
        let spec_trans = SpecularTransmission::new(color, fresnel);
        let bsdf = BSDF::new(vec![Box::new(spec_trans)]);

        Receiver(Arc::new(ReceiverObj::new(sphere, Arc::new(bsdf))))
    }

    fn capsule() -> Instance {
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

        Receiver(Arc::new(ReceiverObj::new(capsule, Arc::new(bsdf))))
    }

    fn tube() -> Instance {
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

        Receiver(Arc::new(ReceiverObj::new(tube, Arc::new(bsdf))))
    }

    fn left_wall() -> Instance {
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

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }

    fn right_wall() -> Instance {
        let aabb = Aabb::new(
            Vec3::new(RIGHT_WALL, FLOOR - THICKNESS, BACK_WALL - THICKNESS),
            Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
        );

        let color = Spectrum::green();
        let oren_nayar = OrenNayar::new(color, SIGMA);
        let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }

    fn back_wall() -> Instance {
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

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }

    fn floor() -> Instance {
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

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }

    fn ceiling() -> Instance {
        let aabb = Aabb::new(
            Vec3::new(LEFT_WALL - THICKNESS, CEILING, BACK_WALL - THICKNESS),
            Vec3::new(RIGHT_WALL + THICKNESS, CEILING + THICKNESS, FRONT),
        );

        let color = Spectrum::white();
        let oren_nayar = OrenNayar::new(color, SIGMA);
        let bsdf = BSDF::new(vec![Box::new(oren_nayar)]);

        Receiver(Arc::new(ReceiverObj::new(aabb, Arc::new(bsdf))))
    }
}

impl DemoScene for CornellScene {
    fn create<'a>(width: u32, height: u32) -> (Scene, Camera) {
        (Self::create_box(), Self::create_camera(width, height))
    }
}
