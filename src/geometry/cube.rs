use ultraviolet::{Mat4, Vec3};

use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry, GeometryInfo, Hit, InversibleExt};

#[allow(dead_code)]
pub struct Cube {
    world_to_local: Mat4,
    local_to_world: Mat4,
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(Aabb::default(), 0.0, 0.0, 0.0)
    }
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

    pub fn point_to_local_space(&self, v: Vec3) -> Vec3 {
        (self.world_to_local * v.into_homogeneous_vector()).xyz()
    }

    pub fn point_to_world_space(&self, v: Vec3) -> Vec3 {
        (self.world_to_local * v.into_homogeneous_vector()).xyz()
    }
}

impl Container for Cube {
    fn contains(&self, obj: Vec3) -> bool {
        let v = self.point_to_local_space(obj);

        Aabb::default().contains(v)
    }
}

impl Geometry for Cube {
    fn bounding_box(&self) -> Aabb {
        unimplemented!();

        // FIXME
        // let unit = Aabb::default();
        // let min = (self.local_to_world * unit.min.into_homogeneous_vector()).xyz();
        // let max = (self.local_to_world * unit.max.into_homogeneous_vector()).xyz();
        // let tmp = min;
        //
        // let min = min.min_by_component(max);
        // let max = max.max_by_component(tmp);
        //
        // Aabb::new(min, max)
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let origin = self.point_to_local_space(ray.origin);
        let direction = self.point_to_local_space(ray.direction);
        let ray = Ray::new(origin, direction, f32::INFINITY);

        Aabb::default().intersect(&ray)
    }

    fn get_info(&self, _hit: Hit) -> GeometryInfo {
        unimplemented!()
    }
}
