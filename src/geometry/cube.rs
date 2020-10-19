use ultraviolet::{Mat4, Vec3, Vec4};

use crate::geometry::aabb::{intersects_unit_aabb, unit_aabb, Aabb};
use crate::geometry::ray::Ray;
use crate::geometry::{Boxable, Intersectable, Intersection, InversibleExt};

pub struct Cube {
    world_to_local: Mat4,
    local_to_world: Mat4,
}

impl Cube {
    #[allow(non_snake_case)]
    pub fn new(aabb: Aabb, roll: f32, pitch: f32, yaw: f32) -> Self {
        // to world space
        // division by 2, since Aabb::unit_aabb() is size (-Vec3::one(), Vec3::one())
        let size_inv = aabb.size() / 2.0;
        let translation_inv = aabb.center();

        let S_inv = Mat4::from_nonuniform_scale(size_inv);
        let R_inv = Mat4::from_euler_angles(roll, pitch, yaw);
        let T_inv = Mat4::from_translation(translation_inv);

        let local_to_world = T_inv * R_inv * S_inv;

        // to local space
        let size = size_inv.inversed();
        let translation = -translation_inv;

        let S = Mat4::from_nonuniform_scale(size);
        let R = Mat4::from_euler_angles(-roll, -pitch, -yaw);
        let T = Mat4::from_translation(translation);

        let world_to_local = T * R * S;

        Self {
            world_to_local,
            local_to_world,
        }
    }
}

impl Boxable for Cube {
    fn bounding_box(&self) -> Option<Aabb> {
        let unit = unit_aabb();
        let min = (self.local_to_world * unit.min.into_homogeneous_vector()).xyz();
        let max = (self.local_to_world * unit.max.into_homogeneous_vector()).xyz();
        let tmp = min;

        let min = min.min_by_component(max);
        let max = max.max_by_component(tmp);

        Some(Aabb::new(min, max))
    }
}

impl Intersectable<Ray> for Cube {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let origin = (self.world_to_local * ray.origin.into_homogeneous_point()).xyz();
        let direction = (self.world_to_local * ray.direction.into_homogeneous_vector()).xyz();

        if let Some(i) = intersects_unit_aabb(&Ray::new(origin, direction)) {
            let position =
                (self.local_to_world * i.position.unwrap().into_homogeneous_point()).xyz();
            let normal = (self.local_to_world * i.normal.unwrap().into_homogeneous_vector()).xyz();
            let t = i.t.unwrap();

            return Some(Intersection::new(position, normal, t));
        }

        None
    }
}
