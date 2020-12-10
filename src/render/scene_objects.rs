use crate::render::light::{Light, LightSample, OcclusionTester};
use crate::render::material::Material;
use crate::render::scene::SceneIntersection;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{DefaultGeometry, Geometry, GeometryInfo};
use ultraviolet::Vec3;

#[derive(Debug)]
pub struct SceneObject {
    shape: Box<dyn Geometry>,
    pub material: Material,
}

impl Default for SceneObject {
    fn default() -> Self {
        Self::new(Box::new(DefaultGeometry), Material::default())
    }
}

impl SceneObject {
    pub fn new(shape: Box<dyn Geometry>, material: Material) -> Self {
        Self { shape, material }
    }
}

impl Geometry for SceneObject {
    fn bounding_box(&self) -> Aabb {
        self.shape.bounding_box()
    }

    fn sample_surface(&self, sample: &Vec3) -> Vec3 {
        self.shape.sample_surface(sample)
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        self.shape.intersect(ray)
    }
}

impl Light for SceneObject {
    fn is_delta_light(&self) -> bool {
        true
    }

    fn sample(&self, intersection: &SceneIntersection) -> LightSample {
        let random = Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32());

        let position = self.sample_surface(&random);
        let dir = position - intersection.info.point;

        let incident = dir.normalized();
        let pdf = 1.0;
        let occlusion_tester = OcclusionTester::new(intersection.info.point, position);

        let intensity = self.material.radiance(&incident, &intersection.info.normal) / dir.mag_sq();

        LightSample::new(intensity, incident, pdf, occlusion_tester)
    }
}
