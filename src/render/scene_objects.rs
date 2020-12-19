use crate::render::material::Material;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Boundable, Geometry, Intersectable, Intersection};
use std::ops::Deref;

#[derive(Debug)]
pub struct Object<T> {
    shape: T,
    pub material: Material,
}

impl<T> Object<T> {
    pub fn new(shape: T, material: Material) -> Self {
        Self { shape, material }
    }
}

impl<T> Boundable for Object<T>
where
    T: Boundable,
{
    fn bounds(&self) -> Aabb {
        self.shape.bounds()
    }
}

impl<T> Intersectable for Object<T>
where
    T: Intersectable,
{
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.shape.intersects(ray)
    }
}

pub trait SceneObject: Geometry {
    fn material(&self) -> &Material;
}
impl<T> SceneObject for Object<T>
where
    T: Geometry,
{
    fn material(&self) -> &Material {
        &self.material
    }
}
impl<T: ?Sized> SceneObject for T
where
    T: Deref + Geometry,
    T::Target: SceneObject,
{
    fn material(&self) -> &Material {
        self.deref().material()
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
