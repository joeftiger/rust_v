use crate::render::light::{Light, LightSample, SampledOcclusionTester, LIGHT_SAMPLE_DELTA};
use crate::render::material::Material;
use crate::render::scene::SceneIntersection;
use crate::{LIGHT_SAMPLES_1D, LIGHT_SAMPLES_3D};
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{DefaultGeometry, Geometry, GeometryInfo};
use ultraviolet::Vec3;
use util::floats;

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

    fn sample(&self, intersection: &SceneIntersection, sample: &Vec3) -> LightSample {
        let mut direction = Vec3::zero();

        let mut rays = [Ray::default(); LIGHT_SAMPLES_3D];
        let mut i = 0;
        for x in 0..LIGHT_SAMPLES_1D {
            for y in 0..LIGHT_SAMPLES_1D {
                for z in 0..LIGHT_SAMPLES_1D {
                    let x_dt = LIGHT_SAMPLE_DELTA * x as f32;
                    let y_dt = LIGHT_SAMPLE_DELTA * y as f32;
                    let z_dt = LIGHT_SAMPLE_DELTA * z as f32;
                    let new_sample = Vec3::new(
                        (sample.x + x_dt) % 1.0,
                        (sample.y + y_dt) % 1.0,
                        (sample.z + z_dt) % 1.0,
                    );

                    let position = self.shape.sample_surface(&new_sample);
                    let dir = position - intersection.info.point;
                    direction += dir;

                    let mut ray = Ray::new(intersection.info.point, dir);
                    ray.t_start = floats::BIG_EPSILON;
                    rays[i] = ray;

                    i += 1;
                }
            }
        }
        debug_assert_eq!(i, LIGHT_SAMPLES_3D);

        direction /= LIGHT_SAMPLES_1D as f32;

        let incident = direction.normalized();
        let pdf = 1.0;
        let occlusion_tester = Box::new(SampledOcclusionTester::new(rays));

        let intensity =
            self.material.radiance(&incident, &intersection.info.normal) / direction.mag_sq();

        LightSample::new(intensity, incident, pdf, occlusion_tester)
    }
}
