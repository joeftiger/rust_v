use crate::render::material::Material;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{DefaultGeometry, Geometry, IntersectionInfo, Boundable, Intersectable};

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

impl Boundable for SceneObject {
    fn bounds(&self) -> Aabb {
        self.shape.bounds()
    }
}

impl Intersectable for SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.shape.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.shape.intersects(ray)
    }
}

// impl Light for SceneObject {
//     fn is_delta_light(&self) -> bool {
//         true
//     }
//
//     fn spectrum(&self) -> Spectrum {
//         self.material.emission.unwrap_or(Spectrum::black())
//     }
//
//     fn sample(&self, intersection: &SceneIntersection, sample: &Vec3) -> Box<dyn LightTester> {
//         debug_assert!(sample.x >= 0.0);
//         debug_assert!(sample.x < 1.0);
//         debug_assert!(sample.y >= 0.0);
//         debug_assert!(sample.y < 1.0);
//         debug_assert!(sample.z >= 0.0);
//         debug_assert!(sample.z < 1.0);
//
//         let area = self.surface_area();
//
//         let mut light_testers = [SimpleLightTester::default(); LIGHT_SAMPLES_3D];
//         let mut i = 0;
//         for x in 0..LIGHT_SAMPLES_1D {
//             for y in 0..LIGHT_SAMPLES_1D {
//                 for z in 0..LIGHT_SAMPLES_1D {
//                     let x_dt = LIGHT_SAMPLE_DELTA * x as f32;
//                     let y_dt = LIGHT_SAMPLE_DELTA * y as f32;
//                     let z_dt = LIGHT_SAMPLE_DELTA * z as f32;
//                     let new_sample = Vec3::new(
//                         (sample.x + x_dt) % 1.0,
//                         (sample.y + y_dt) % 1.0,
//                         (sample.z + z_dt) % 1.0,
//                     );
//
//                     let position = self.shape.sample_surface(&new_sample);
//
//                     light_testers[i] =
//                         SimpleLightTester::new(area, intersection.info.point, position);
//                     i += 1;
//                 }
//             }
//         }
//         debug_assert_eq!(i, LIGHT_SAMPLES_3D);
//
//         Box::new(SampledLightTester::new(light_testers))
//     }
// }
