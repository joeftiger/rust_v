use std::ops::Mul;

use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;

pub mod aabb;
pub mod cylinder;
pub mod triangle;
pub mod lens;
pub mod mesh;
pub mod plane;
pub mod sphere;

/// An intersection consists of a position and a normal, therefore allowing calculations like
/// reflection and refraction.
#[derive(Debug, Default)]
pub struct Intersection {
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
    pub t: Option<f32>,
}

impl Intersection {
    pub fn new(position: Vec3, normal: Vec3, t: f32) -> Self {
        Self {
            position: Some(position),
            normal: Some(normal),
            t: Some(t),
        }
    }
}

/// A trait that allows measuring the angle between two structs.
/// For example:
/// ```
/// use ultraviolet::Vec3;
/// use rust_v::geometry::AngularExt;
///
/// let v1 = Vec3::unit_x();
/// let v2 = Vec3::unit_y();
/// let angle = v1.angle_to(v2);
///
/// assert_eq!(angle, 90 * 180 / f32::PI);  // 90 degrees in radians
/// ```
pub trait AngularExt<T> {
    /// Returns the angle to the other in radians.
    #[must_use]
    fn angle_to(&self, other: T) -> f32;
}

/// A trait that allows the implementation to ceil / floor itself, such that e.g.:
/// ```rust
/// use ultraviolet::Vec3;
/// use rust_v::geometry::CeilFloorExt;
///
/// let v = Vec3::new(0.7, 0.7, 0.7);
///
/// assert_eq!(Vec3::one(), v.ceil());
/// assert_eq!(Vec3::zero(), v.floor());
/// ```
pub trait CeilFloorExt {
    fn ceil(&self) -> Self;

    #[must_use]
    fn floor(&self) -> Self;
}

/// This trait allows an intersector to check for an intersection with the implementation.
/// For example:
/// ```
/// use rust_v::geometry::aabb::Aabb;
/// use ultraviolet::Vec3;
/// use rust_v::geometry::{Ray, Intersectable};
///
/// let aabb = Aabb::new(-Vec3::one(), Vec3::one());
/// let ray = Ray::new(-Vec3::unit_x() * 2, Vec3::unit_x());
/// let intersection = aabb.intersects(&ray);
///
/// assert!(intersection.is_some());
/// ```
pub trait Intersectable<T> {
    #[must_use]
    fn intersects(&self, intersector: T) -> Option<Intersection>;
}

/// Allows an implementation to be put in a "box" (aabb), if available.
pub trait Boxable {
    #[must_use]
    fn bounding_box(&self) -> Option<Aabb>;
}

/// Allows itself to be inversed.
/// For example:
/// ```
/// use ultraviolet::Vec3;
/// use rust_v::geometry::InversibleExt;
///
/// let v = Vec3::one() * 2.0;
/// let inverse = v.inversed();
///
/// assert_eq!(inverse, Vec3::one() / 2.0);
/// ```
pub trait InversibleExt {
    #[must_use]
    fn inversed(&self) -> Self;
}

impl AngularExt<Self> for Vec3 {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Self) -> f32 {
        f32::acos(self.dot(other) / (self.mag() * other.mag()))
    }
}

impl AngularExt<Ray> for Vec3 {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Ray) -> f32 {
        self.angle_to(other.direction)
    }
}

impl CeilFloorExt for Vec3 {
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

impl InversibleExt for Vec3 {
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

impl AngularExt<Self> for Ray {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Ray) -> f32 {
        self.direction.angle_to(other.direction)
    }
}

impl AngularExt<Vec3> for Ray {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Vec3) -> f32 {
        self.direction.angle_to(other)
    }
}

impl Mul<f32> for Ray {
    type Output = Vec3;

    #[inline]
    #[must_use]
    fn mul(self, rhs: f32) -> Self::Output {
        self.at(rhs)
    }
}
