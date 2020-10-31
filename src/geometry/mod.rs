use crate::floats;
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::{Ray, Ray4};
use ultraviolet::{f32x4, Vec3, Vec3x4};

pub mod aabb;
pub mod cube;
pub mod cylinder;
pub mod lens;
pub mod mesh;
pub mod point;
pub mod ray;
pub mod sphere;
mod tests;

macro_rules! hits {
    ($($name:ident => $ray:ident, $float:ident), +) => {
        $(
            #[derive(Copy, Clone, PartialEq)]
            pub struct $name {
                pub ray: $ray,
                pub t: $float,
            }

            impl $name {
                pub fn new(ray: $ray, t: $float) -> Self {
                    Self { ray, t }
                }
            }
        )+
    };
}

hits!(
    Hit => Ray, f32,
    Hit4 => Ray4, f32x4
);

impl From<Ray> for Hit {
    fn from(ray: Ray) -> Self {
        Self::new(ray, ray.t)
    }
}

impl From<Ray4> for Hit4 {
    fn from(ray: Ray4) -> Self {
        Self::new(ray, ray.t)
    }
}

macro_rules! geometry_info {
    ($($name:ident => $hit:ident, $ray:ident, $float:ident, $vec:ident, $offset_epsilon:expr), +) => {
        $(
            /// Consists of:
            /// - ray: Ray
            /// - t, offset_epsilon: f32
            /// - normal: Vec3
            #[derive(Copy, Clone)]
            pub struct $name {
                pub ray: $ray,
                pub t: $float,
                pub point: $vec,
                pub normal: $vec,
                pub offset_epsilon: $float,
            }

            impl $name {
                pub fn new(hit: $hit, point: $vec, normal: $vec) -> Self {
                    Self { ray: hit.ray, t: hit.t, point, normal, offset_epsilon: $offset_epsilon }
                }

                /// Creates a ray from `self.point` into the given direction, offset by `self.offset_epsilon`.
                pub fn create_ray(&self, dir: $vec) -> $ray {
                    let mut ray = self.ray;
                    ray.origin = self.point + self.normal * self.offset_epsilon;
                    ray.direction = dir;

                    ray
                }
            }
        )+
    };
}

geometry_info!(
    GeometryInfo => Hit, Ray, f32, Vec3, floats::DEFAULT_EPSILON,
    GeometryInfox4 => Hit4, Ray4, f32x4, Vec3x4, f32x4::splat(floats::DEFAULT_EPSILON)
);

impl PartialEq for GeometryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.ray == other.ray
            && self.t == other.t
            && self.point == other.point
            && self.normal == other.normal
    }
}

impl PartialEq for GeometryInfox4 {
    fn eq(&self, other: &Self) -> bool {
        self.ray == other.ray
            && self.t == other.t
            && self.point.x == other.point.x
            && self.point.y == other.point.y
            && self.point.z == other.point.z
            && self.normal.x == other.normal.x
            && self.normal.y == other.normal.y
            && self.normal.z == other.normal.z
    }
}

pub trait Container {
    fn contains(&self, obj: Vec3) -> bool;
}

pub trait Geometry: Send + Sync {
    fn bounding_box(&self) -> Aabb;

    fn intersect(&self, ray: &Ray) -> Option<f32>;

    fn get_info(&self, hit: Hit) -> GeometryInfo;
}

/// A trait that allows measuring the angle between two structs.
/// For example:
/// ```rust
/// use ultraviolet::Vec3;
/// use rust_v::geometry::AngularExt;
/// use rust_v::floats;
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
/// ```rust
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

/// Allows itself to be strictly compared to another self.
/// For example:
/// ```rust
/// use ultraviolet::Vec3;
/// use rust_v::geometry::ComparableExt;
///
/// let v = Vec3::zero();
/// let other = Vec3::one();
///
/// assert!(v.lt(&other));
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
