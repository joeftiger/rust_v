use crate::geometry::aabb::Aabb;
use ultraviolet::Vec3;

pub mod aabb;
pub mod cube;
pub mod cylinder;
pub mod intersection;
pub mod lens;
pub mod ray;
pub mod sphere;

pub trait Container<Tin, Tout> {
    fn contains(&self, obj: Tin) -> Tout;
}

pub trait Geometry<Tin, Tout> {
    fn bounding_box(&self) -> Aabb;

    fn intersect(&self, ray: &Tin) -> Option<Tout>;
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

impl AngularExt<Self> for Vec3 {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Self) -> f32 {
        f32::acos(self.dot(other) / (self.mag() * other.mag()))
    }
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
