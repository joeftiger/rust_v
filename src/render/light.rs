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

pub trait OcclusionTester {
    fn unoccluded(&self, scene: &Scene) -> bool;
}

pub struct SimpleOcclusionTester {
    ray: Ray,
}

impl SimpleOcclusionTester {
    pub fn new(from: Vec3, to: Vec3) -> Self {
        let mut ray = Ray::in_range(&from, &to);
        ray.t_start = floats::BIG_EPSILON;
        Self { ray }
    }
}

impl OcclusionTester for SimpleOcclusionTester {
    fn unoccluded(&self, scene: &Scene) -> bool {
        !scene.is_occluded(&self.ray)
    }
}

pub struct SampledOcclusionTester {
    rays: [Ray; LIGHT_SAMPLES_3D],
}

impl SampledOcclusionTester {
    pub fn new(rays: [Ray; LIGHT_SAMPLES_3D]) -> Self {
        Self { rays }
    }
}

impl OcclusionTester for SampledOcclusionTester {
    fn unoccluded(&self, scene: &Scene) -> bool {
        self.rays.iter().any(|ray| !scene.is_occluded(ray))
    }
}

pub struct LightSample {
    pub spectrum: Spectrum,
    pub incident: Vec3,
    pub pdf: f32,
    pub occlusion_tester: Box<dyn OcclusionTester>,
}

impl LightSample {
    pub fn new(
        spectrum: Spectrum,
        incident: Vec3,
        pdf: f32,
        occlusion_tester: Box<dyn OcclusionTester>,
    ) -> Self {
        Self {
            spectrum,
            incident,
            pdf,
            occlusion_tester,
        }
    }
}

pub trait Light: Send + Sync {
    fn is_delta_light(&self) -> bool;

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

    pub fn direction_from(&self, point: &Vec3) -> Vec3 {
        (self.position - *point).normalized()
    }

    pub fn direction_to(&self, point: &Vec3) -> Vec3 {
        (*point - self.position).normalized()
    }

    pub fn distance(&self, point: &Vec3) -> f32 {
        (self.position - *point).mag()
    }
}

impl Light for PointLight {
    fn is_delta_light(&self) -> bool {
        true
    }

    fn sample(&self, intersection: &SceneIntersection, _: &Vec3) -> LightSample {
        let dir = self.position - intersection.info.point;

        let incident = dir.normalized();
        let pdf = 1.0;
        let occlusion_tester = Box::new(SimpleOcclusionTester::new(
            intersection.info.point,
            self.position,
        ));

        let intensity = self.intensity / dir.mag_sq();

        LightSample::new(intensity, incident, pdf, occlusion_tester)
    }
}
