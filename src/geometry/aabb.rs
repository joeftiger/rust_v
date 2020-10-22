use ultraviolet::{Vec3, Vec3x4, f32x4};
use crate::geometry::{Container, Geometry};
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::floats;

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
        self.min.x <= self.max.x &&
            self.min.y <= self.max.y &&
            self.min.z <= self.max.z
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

impl Container<Vec3, bool> for Aabb {
    fn contains(&self, obj: Vec3) -> bool {
        let clamped = obj.clamped(self.min, self.max);

        clamped == obj
    }
}

impl Container<Vec3x4, f32x4> for Aabb {
    fn contains(&self, obj: Vec3x4) -> f32x4 {
        let min = Vec3x4::splat(self.min);
        let max = Vec3x4::splat(self.max);

        let clamped = obj.clamped(min, max);

        f32x4::from([
            if clamped[0] == obj[0] { f32::from_bits(u32::MAX) } else {0.0},
            if clamped[1] == obj[1] { f32::from_bits(u32::MAX) } else {0.0},
            if clamped[2] == obj[2] { f32::from_bits(u32::MAX) } else {0.0},
            if clamped[3] == obj[3] { f32::from_bits(u32::MAX) } else {0.0}
        ])
    }
}

impl Geometry<Ray, Intersection> for Aabb {
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        if t_max < 0.0 || t_max < t_min {
            return None;
        }

        let position = ray.at(t_min);
        let normal: Vec3;

        if floats::approx_equal(position.x, self.min.x) {
            normal = -Vec3::unit_x();
        } else if floats::approx_equal(position.x, self.max.x) {
            normal = Vec3::unit_x();
        } else if floats::approx_equal(position.y, self.min.y) {
            normal = -Vec3::unit_y();
        } else if floats::approx_equal(position.y, self.max.y) {
            normal = Vec3::unit_y();
        } else if floats::approx_equal(position.z, self.min.z) {
            normal = -Vec3::unit_z();
        } else {
            normal = Vec3::unit_z();
        }

        Some(Intersection::new(t_min, position, normal))
    }
}

impl Default for Aabb {
    fn default() -> Self {
        let min = -Vec3::one();
        let max = Vec3::one();

        Self::new(min, max)
    }
}
