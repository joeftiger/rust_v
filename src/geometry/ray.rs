use std::fmt::Debug;

use ultraviolet::vec::Vec3;

use crate::geometry::AngularExt;
use crate::Float;

#[derive(Copy, Clone, Debug, Default)]
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
    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction * t as f32
    }

    #[inline]
    #[must_use]
    pub fn distance(&self, p: Vec3) -> Float {
        (p - self.origin).mag() as Float
    }
}

impl AngularExt<Self> for Ray {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Self) -> Float {
        self.direction.angle_to(other.direction)
    }
}
