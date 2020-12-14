#![allow(dead_code)]
#![allow(unused_variables)]

use ultraviolet::Vec3;

use crate::render::scene::{Scene, SceneIntersection};
use crate::{Spectrum, LIGHT_SAMPLES_1D, LIGHT_SAMPLES_3D};
use geometry::ray::Ray;
use util::floats;

pub const LIGHT_SAMPLE_DELTA: f32 = 1.0 / LIGHT_SAMPLES_1D as f32;

bitflags! {
    pub struct LightType: u8 {
        const DELTA_POSITION = 1 << 0;
        const DELTA_DIRECTION = 1 << 1;
        const AREA = 1 << 2;
        const INFINITY = 1 << 3;
    }
}

pub trait LightTester {
    fn test(&self, scene: &Scene) -> Option<(f32, Vec3)>;
}

#[derive(Copy, Clone, Default)]
pub struct SimpleLightTester {
    intensity: f32,
    ray: Ray,
}

impl SimpleLightTester {
    pub fn new(from: Vec3, to: Vec3) -> Self {
        let intensity = 1.0 / (to - from).mag_sq();

        let mut ray = Ray::in_range(&from, &to);
        ray.t_start = floats::BIG_EPSILON;
        ray.t_end -= floats::BIG_EPSILON;

        Self { intensity, ray }
    }
}

impl LightTester for SimpleLightTester {
    fn test(&self, scene: &Scene) -> Option<(f32, Vec3)> {
        if scene.is_occluded(&self.ray) {
            None
        } else {
            Some((self.intensity, self.ray.direction))
        }
    }
}

pub struct SampledLightTester {
    light_testers: [SimpleLightTester; LIGHT_SAMPLES_3D],
}

impl SampledLightTester {
    pub fn new(light_testers: [SimpleLightTester; LIGHT_SAMPLES_3D]) -> Self {
        Self { light_testers }
    }
}

impl LightTester for SampledLightTester {
    fn test(&self, scene: &Scene) -> Option<(f32, Vec3)> {
        self.light_testers.iter().filter_map(|t| t.test(scene)).next()
    }
}

pub struct LightSample {
    pub pdf: f32,
    pub light_tester: Box<dyn LightTester>,
}

impl LightSample {
    pub fn new(
        pdf: f32,
        light_tester: Box<dyn LightTester>,
    ) -> Self {
        Self {
            pdf,
            light_tester,
        }
    }
}

pub trait Light: Send + Sync {
    fn is_delta_light(&self) -> bool;

    fn spectrum(&self) -> Spectrum;

    fn sample(&self, intersection: &SceneIntersection, sample: &Vec3) -> LightSample;
}

#[derive(Debug)]
pub struct PointLight {
    pub position: Vec3,
    pub intensity: Spectrum,
}

impl PointLight {
    pub fn new(position: Vec3, intensity: Spectrum) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn is_delta_light(&self) -> bool {
        true
    }

    fn spectrum(&self) -> Spectrum {
        self.intensity
    }

    fn sample(&self, intersection: &SceneIntersection, _: &Vec3) -> LightSample {
        let pdf = 1.0;
        let occlusion_tester = Box::new(SimpleLightTester::new(
            intersection.info.point,
            self.position,
        ));

        LightSample::new(pdf, occlusion_tester)
    }
}
