use crate::floats;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry, GeometryInfo, Hit};
use ultraviolet::Vec3;

#[derive(Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        debug_assert!(min.x <= max.x);
        debug_assert!(min.y <= max.y);
        debug_assert!(min.z <= max.z);

        Self { min, max }
    }

    /// Be careful with this one!
    /// It can be used to outer join many aabbs with each other.
    pub fn inverted_infinite() -> Self {
        let min = Vec3::one() * f32::INFINITY;
        let max = Vec3::one() * f32::NEG_INFINITY;
        Self { min, max }
    }

    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3 {
        (self.max + self.min) / 2.0
    }

    /// Creates the inner join / intersection of both aabbs.
    pub fn inner_join(&self, other: &Self) -> Self {
        let min = self.min.max_by_component(other.min);
        let max = self.max.min_by_component(other.max);

        Self::new(min, max)
    }

    /// Creates the outer join / overlap of both aabbs.
    pub fn outer_join(&self, other: &Self) -> Self {
        let min = self.min.min_by_component(other.min);
        let max = self.max.max_by_component(other.max);

        Self::new(min, max)
    }
}

impl Container for Aabb {
    fn contains(&self, obj: Vec3) -> bool {
        let clamped = obj.clamped(self.min, self.max);

        clamped == obj
    }
}

impl Geometry for Aabb {
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        if t_max < 0.0 || t_max < t_min {
            None
        } else {
            Some(t_min)
        }
    }

    fn get_info(&self, hit: Hit) -> GeometryInfo {
        let position = hit.ray.at(hit.t);
        let normal: Vec3;

        if floats::approx_equal_big(position.x, self.min.x) {
            normal = -Vec3::unit_x();
        } else if floats::approx_equal_big(position.x, self.max.x) {
            normal = Vec3::unit_x();
        } else if floats::approx_equal_big(position.y, self.min.y) {
            normal = -Vec3::unit_y();
        } else if floats::approx_equal_big(position.y, self.max.y) {
            normal = Vec3::unit_y();
        } else if floats::approx_equal_big(position.z, self.min.z) {
            normal = -Vec3::unit_z();
        } else if floats::approx_equal_big(position.z, self.max.z) {
            normal = Vec3::unit_z();
        } else {
            panic!("f32 epsilon too small!");
        }

        GeometryInfo::new(hit, position, normal)
    }
}

impl Default for Aabb {
    fn default() -> Self {
        let min = -Vec3::one();
        let max = Vec3::one();

        Self::new(min, max)
    }
}
