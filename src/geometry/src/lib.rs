pub mod aabb;
pub mod bvh;
pub mod capsule;
pub mod cylinder;
pub mod lens;
pub mod mesh;
pub mod point;
pub mod ray;
pub mod sphere;
mod tests;
pub mod tube;

use crate::aabb::Aabb;
use crate::ray::Ray;
use std::fmt::Debug;
use ultraviolet::Vec3;
use util::{floats, MinMaxExt};

#[inline]
pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vec3 {
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub ray: Ray,
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Intersection {
    pub fn new(ray: Ray, t: f32, point: Vec3, normal: Vec3) -> Self {
        Self { ray, t, point, normal }
    }

    pub fn create_ray(&self, dir: Vec3) -> Ray {
        let t_start = floats::BIG_EPSILON;
        let t_end = f32::INFINITY;

        Ray::with(self.point, dir, t_start, t_end)
    }
}

impl MinMaxExt for Intersection {
    fn mmin(&self, other: &Self) -> Self {
        if self.t <= other.t {
            return *self;
        }

        *other
    }

    fn mmax(&self, other: &Self) -> Self {
        if self.t >= other.t {
            return *self;
        }

        *other
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.point == other.point && self.normal == other.normal
    }
}

/// A trait for objects that can report an aabb as their bounds.
pub trait Boundable {
    /// The bounds of this object.
    fn bounds(&self) -> Aabb;
}

/// A trait for objects that can contain a point or position.
pub trait Container<T = Vec3> {
    /// Returns whether the given obj is inside this container.
    fn contains(&self, obj: &T) -> bool;
}

/// A trait for objects that can be intersected by rays.
pub trait Intersectable<T = Ray> {
    /// Intersects the given ray with this object.
    fn intersect(&self, ray: &T) -> Option<Intersection>;

    /// Returns whether the given ray intersects with this object.
    ///
    /// This function should be overwritten to specialize instead of checking whether
    /// `intersect(ray)` is `Some`.
    fn intersects(&self, ray: &T) -> bool {
        self.intersect(ray).is_some()
    }
}

/// A helper trait to combine both `Boundable` and `Intersectable` in a threadsafe way
/// (`Send` + `Sync`), allowing `Debug` prints.
pub trait Geometry: Debug + Boundable + Intersectable + Send + Sync {}
impl<T: ?Sized> Geometry for T where T: Debug + Boundable + Intersectable + Send + Sync {}

#[derive(Debug, PartialEq)]
pub struct DefaultGeometry;

impl Boundable for DefaultGeometry {
    fn bounds(&self) -> Aabb {
        Aabb::inverted_infinite()
    }
}

impl Intersectable for DefaultGeometry {

    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        None
    }

    fn intersects(&self, _: &Ray) -> bool {
        false
    }
}

/// A trait that allows measuring the angle between two structs.
/// For example:
/// ```rust
/// use ultraviolet::Vec3;
/// use geometry::AngularExt;
/// use util::floats;
///
/// let v1 = Vec3::unit_x();
/// let v2 = Vec3::unit_y();
/// let angle = v1.angle_to(&v2);
///
/// assert!(floats::approx_equal(angle, std::f32::consts::FRAC_PI_2));  // 90 degrees in radians
/// ```
pub trait AngularExt {
    /// Returns the angle to the other in radians.
    #[must_use]
    fn angle_to(&self, other: &Self) -> f32;
}

impl AngularExt for Vec3 {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: &Self) -> f32 {
        f32::acos(self.dot(*other) / (self.mag() * other.mag()))
    }
}

/// A trait that allows ceiling / flooring itself, such that e.g.:
/// ```rust
/// use ultraviolet::Vec3;
/// use geometry::CeilFloorExt;
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
/// ```rust
/// use ultraviolet::Vec3;
/// use geometry::InversibleExt;
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

/// Allows itself to be strictly compared to another self.
/// For example:
/// ```rust
/// use ultraviolet::Vec3;
/// use geometry::ComparableExt;
///
/// let v = Vec3::zero();
/// let other = Vec3::one();
///
/// assert!(v.lt(&other));
/// assert!(other.gt(&v));
/// ```
pub trait ComparableExt {
    #[must_use]
    fn lt(&self, other: &Self) -> bool;

    #[must_use]
    fn leq(&self, other: &Self) -> bool;

    #[must_use]
    fn gt(&self, other: &Self) -> bool;

    #[must_use]
    fn geq(&self, other: &Self) -> bool;

    #[must_use]
    fn eq(&self, other: &Self) -> bool;
}

impl ComparableExt for Vec3 {
    fn lt(&self, other: &Self) -> bool {
        self.x < other.x && self.y < other.y && self.z < other.z
    }

    fn leq(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z
    }

    fn gt(&self, other: &Self) -> bool {
        self.x > other.x && self.y > other.y && self.z > other.z
    }

    fn geq(&self, other: &Self) -> bool {
        self.x >= other.x && self.y >= other.y && self.z >= other.z
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

/// Allows the distance calculation to another self.
/// For example:
/// ```rust
/// use ultraviolet::Vec3;
/// use geometry::DistanceExt;
///
/// let v = Vec3::zero();
/// let other = Vec3::unit_x();
///
/// assert_eq!(1.0, v.distance(&other));
/// ```
pub trait DistanceExt {
    fn distance(&self, other: &Self) -> f32;
}

impl DistanceExt for Vec3 {
    fn distance(&self, other: &Self) -> f32 {
        (*other - *self).mag()
    }
}

pub struct CoordinateSystem {
    pub e1: Vec3,
    pub e2: Vec3,
    pub e3: Vec3,
}

impl CoordinateSystem {
    pub fn new(e1: Vec3, e2: Vec3, e3: Vec3) -> Self {
        Self { e1, e2, e3 }
    }
}

impl From<&Vec3> for CoordinateSystem {
    /// Creates a local ortho-normal coordinate system from the given vector.
    fn from(e1: &Vec3) -> Self {
        let e2 = if e1.x.abs() > e1.y.abs() {
            let inv = 1.0 / f32::sqrt(e1.x * e1.x + e1.z * e1.z);
            Vec3::new(-e1.z * inv, 0.0, e1.x * inv)
        } else {
            let inv = 1.0 / f32::sqrt(e1.y * e1.y + e1.z * e1.z);
            Vec3::new(0.0, e1.z * inv, -e1.y * inv)
        };
        let e3 = e1.cross(e2);

        Self::new(*e1, e2, e3)
    }
}
