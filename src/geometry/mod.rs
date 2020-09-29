use std::ops::Mul;

use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;

pub mod plane;
pub mod sphere;
pub mod aabb;

pub struct Intersection {
    pub hit: bool,
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
}

impl Intersection {
    pub fn none() -> Self {
        Self {
            hit: false,
            position: None,
            normal: None,
        }
    }

    pub fn only_hit(hit: bool) -> Self {
        Self {
            hit,
            position: None,
            normal: None,
        }
    }

    pub fn at(position: Vec3, normal: Vec3) -> Self {
        Self {
            hit: true,
            position: Some(position),
            normal: Some(normal),
        }
    }
}

/// A trait that allows measuring the angle between two same structs.
pub trait Angular<T> {
    /// Returns the angle to the other self in radians.
    #[must_use]
    fn angle_to(&self, other: T) -> f32;
}

/// A trait that allows the implementation to ceil / floor itself, such that e.g.:
/// ```rust
/// use ultraviolet::Vec3;
/// use rust_v::geometry::CeilFloor;
///
/// let v = Vec3::new(0.7, 0.7, 0.7);
///
/// assert_eq!(Vec3::one(), v.ceil());
/// assert_eq!(Vec3::zero(), v.floor());
/// ```
pub trait CeilFloor {
    #[must_use]
    fn ceil(&self) -> Self;

    #[must_use]
    fn floor(&self) -> Self;
}

pub trait Intersectable<T> {
    #[must_use]
    fn intersects(&self, intersector: T) -> Intersection;
}

pub trait Boxable {
    #[must_use]
    fn bounding_box(&self) -> Option<Aabb>;
}

pub trait Inversible {
    #[must_use]
    fn inversed(&self) -> Self;
}

impl Angular<Vec3> for Vec3 {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Vec3) -> f32 {
        f32::acos(self.dot(other) / (self.mag() * other.mag()))
    }
}

impl CeilFloor for Vec3 {
    fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }
}

impl Inversible for Vec3 {
    #[inline]
    #[must_use]
    fn inversed(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }
}

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
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    #[inline]
    #[must_use]
    pub fn distance(self, p: Vec3) -> f32 {
        (p - self.origin).mag()
    }
}

impl Mul<f32> for Ray {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.at(rhs)
    }
}
