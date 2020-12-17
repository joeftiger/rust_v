#![allow(dead_code)]
#![allow(unused_variables)]

use geometry::ray::Ray;
use ultraviolet::Vec3;
use util::floats;

use crate::render::scene::{Scene, SceneIntersection};
use crate::{Spectrum, LIGHT_SAMPLES_1D, LIGHT_SAMPLES_3D};

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
    fn test(&self, scene: &Scene) -> Option<LightSample>;
}

#[derive(Copy, Clone, Default)]
pub struct SimpleLightTester {
    area: f32,
    ray: Ray,
}

impl SimpleLightTester {
    pub fn new(area: f32, from: Vec3, to: Vec3) -> Self {
        debug_assert!(!from.x.is_nan());
        debug_assert!(!from.y.is_nan());
        debug_assert!(!from.z.is_nan());
        debug_assert!(!to.x.is_nan());
        debug_assert!(!to.y.is_nan());
        debug_assert!(!to.z.is_nan());

        let mut ray = Ray::in_range(&from, &to);
        ray.t_start = floats::BIG_EPSILON;
        ray.t_end -= floats::BIG_EPSILON;

        Self { area, ray }
    }
}

impl LightTester for SimpleLightTester {
    fn test(&self, scene: &Scene) -> Option<LightSample> {
        if let Some(i) = scene.intersect(&self.ray) {
            let dist_sq = i.info.t * i.info.t;
            let abs_dot = i.info.normal.dot(-self.ray.direction).abs();
            let pdf = dist_sq / (abs_dot * self.area);

            Some(LightSample::new(pdf, i.info.t, -self.ray.direction))
        } else {
            None
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
    fn test(&self, scene: &Scene) -> Option<LightSample> {
        self.light_testers
            .iter()
            .filter_map(|t| t.test(scene))
            .next()
    }
}

pub struct LightSample {
    pub pdf: f32,
    pub intensity: f32,
    pub incident: Vec3,
}

impl LightSample {
    pub fn new(pdf: f32, intensity: f32, incident: Vec3) -> Self {
        Self {
            pdf,
            intensity,
            incident,
        }
    }
}

pub trait Light: Send + Sync {
    fn is_delta_light(&self) -> bool;

    fn spectrum(&self) -> Spectrum;

    fn sample<'a>(&self, intersection: &SceneIntersection, sample: &Vec3) -> Box<dyn LightTester>;
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

    fn sample(&self, intersection: &SceneIntersection, _: &Vec3) -> Box<dyn LightTester> {
        let pdf = 1.0;

        Box::new(SimpleLightTester::new(
            1.0,
            intersection.info.point,
            self.position,
        ))
    }
}
