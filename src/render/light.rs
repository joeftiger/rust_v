#![allow(dead_code)]
#![allow(unused_variables)]

use ultraviolet::Vec3;

use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Geometry, GeometryInfo};
use std::sync::Arc;
use util::floats;

bitflags! {
    pub struct LightType: u8 {
        const DELTA_POSITION = 1 << 0;
        const DELTA_DIRECTION = 1 << 1;
        const AREA = 1 << 2;
        const INFINITY = 1 << 3;
    }
}

pub struct OcclusionTester {
    ray: Ray,
}

impl OcclusionTester {
    pub fn new(from: Vec3, to: Vec3) -> Self {
        let mut ray = Ray::in_range(&from, &to);
        ray.t_start = floats::BIG_EPSILON;
        Self { ray }
    }

    pub fn unoccluded(&self, scene: &Scene) -> bool {
        !scene.is_occluded(&self.ray)
    }

    // fn transmittance(&self, scene: &Scene, sampler: &Sampler) -> Spectrum;
}

pub struct LightSample {
    pub spectrum: Spectrum,
    pub incident: Vec3,
    pub pdf: f32,
    pub occlusion_tester: OcclusionTester,
}

impl LightSample {
    pub fn new(
        spectrum: Spectrum,
        incident: Vec3,
        pdf: f32,
        visibility_tester: OcclusionTester,
    ) -> Self {
        Self {
            spectrum,
            incident,
            pdf,
            occlusion_tester: visibility_tester,
        }
    }
}

pub trait Light: Send + Sync {
    fn is_delta_light(&self) -> bool;

    fn sample(&self, intersection: &SceneIntersection) -> LightSample;
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

    fn sample(&self, intersection: &SceneIntersection) -> LightSample {
        let dir = self.position - intersection.info.point;

        let incident = dir.normalized();
        let pdf = 1.0;
        let occlusion_tester = OcclusionTester::new(intersection.info.point, self.position);

        let intensity = self.intensity / dir.mag_sq();

        LightSample::new(intensity, incident, pdf, occlusion_tester)
    }
}

#[derive(Debug)]
pub struct Emitter {
    geometry: Arc<dyn Geometry>,
    pub emission: Spectrum,
}

impl Emitter {
    pub fn new(geometry: Arc<dyn Geometry>, emission: Spectrum) -> Self {
        Self { geometry, emission }
    }
}

impl Geometry for Emitter {
    fn bounding_box(&self) -> Aabb {
        self.geometry.bounding_box()
    }

    fn sample_surface(&self, sample: &Vec3) -> Vec3 {
        self.geometry.sample_surface(sample)
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        self.geometry.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.geometry.intersects(ray)
    }
}

impl Light for Emitter {
    fn is_delta_light(&self) -> bool {
        false
    }

    fn sample(&self, intersection: &SceneIntersection) -> LightSample {
        unimplemented!()
    }
}
