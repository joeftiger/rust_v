use crate::render::light::{Light, LightSample, SampledLightTester, LIGHT_SAMPLE_DELTA, SimpleLightTester};
use crate::render::material::Material;
use crate::render::scene::SceneIntersection;
use crate::{LIGHT_SAMPLES_1D, LIGHT_SAMPLES_3D, Spectrum};
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{DefaultGeometry, Geometry, GeometryInfo};
use ultraviolet::Vec3;
use color::Color;

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

    fn spectrum(&self) -> Spectrum {
        self.material.emission.unwrap_or(Spectrum::black())
    }

    fn sample(&self, intersection: &SceneIntersection, sample: &Vec3) -> LightSample {
        debug_assert!(sample.x >= 0.0);
        debug_assert!(sample.x < 1.0);
        debug_assert!(sample.y >= 0.0);
        debug_assert!(sample.y < 1.0);
        debug_assert!(sample.z >= 0.0);
        debug_assert!(sample.z < 1.0);

        let mut light_testers = [SimpleLightTester::default(); LIGHT_SAMPLES_3D];
        let mut i = 0;
        for x in 0..LIGHT_SAMPLES_1D {
            for y in 0..LIGHT_SAMPLES_1D {
                for z in 0..LIGHT_SAMPLES_1D {

                    // FIXME: This does not work correctly, makes weird shapes
                    let x_dt = LIGHT_SAMPLE_DELTA * x as f32;
                    let y_dt = LIGHT_SAMPLE_DELTA * y as f32;
                    let z_dt = LIGHT_SAMPLE_DELTA * z as f32;
                    let new_sample = Vec3::new(
                        (sample.x + x_dt) % 1.0,
                        (sample.y + y_dt) % 1.0,
                        (sample.z + z_dt) % 1.0,
                    );

                    // let norm = (*sample * (2.0 * fastrand::f32() - 1.0));
                    //
                    // let new_sample = norm * LIGHT_SAMPLE_DELTA as f32;

                    let position = self.shape.sample_surface(&new_sample);

                    light_testers[i] = SimpleLightTester::new(intersection.info.point, position);
                    i += 1;
                }
            }
        }
        debug_assert_eq!(i, LIGHT_SAMPLES_3D);

        let pdf = 1.0;
        let light_tester = Box::new(SampledLightTester::new(light_testers));


        LightSample::new(pdf, light_tester)
    }
}
