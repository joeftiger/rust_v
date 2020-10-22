use std::fmt::Debug;

use ultraviolet::vec::Vec3;

use crate::geometry::AngularExt;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    #[inline]
    #[must_use]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    #[inline]
    #[must_use]
    pub fn distance(&self, p: Vec3) -> f32 {
        (p - self.origin).mag()
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::unit_x())
    }
}

impl AngularExt<Self> for Ray {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Self) -> f32 {
        self.direction.angle_to(other.direction)
    }
}
