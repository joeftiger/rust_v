pub mod aabb;
pub mod capsule;
pub mod cube;
pub mod cylinder;
pub mod lens;
pub mod mesh;
pub mod plane;
pub mod point;
pub mod ray;
pub mod sphere;
mod tests;
pub mod tube;

use crate::aabb::Aabb;
use crate::ray::{Ray, Ray4};
use ultraviolet::{f32x4, Vec3, Vec3x4};
use util::{floats, MinMaxExt};

#[inline]
pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vec3 {
    Vec3::new(
        sin_theta * phi.cos(),
        sin_theta * phi.sin(),
        cos_theta,
    )
}

macro_rules! geometry_info {
    ($($name:ident => $ray:ident, $float:ident, $vec:ident), +) => {
        $(
            /// Consists of:
            /// - ray: Ray
            /// - t, offset_epsilon: f32
            /// - point, normal: Vec3
            #[derive(Copy, Clone)]
            pub struct $name {
                pub ray: $ray,
                pub t: $float,
                pub point: $vec,
                pub normal: $vec,
            }

            impl $name {
                pub fn new(ray: $ray, t: $float, point: $vec, normal: $vec) -> Self {
                    Self { ray, t, point, normal }
                }

                /// Creates a ray from `self.point` into the given direction, offset by `floats::EPSILON`.
                pub fn create_ray(&self, direction: $vec) -> $ray {
                    let t_start = $float::from(floats::BIG_EPSILON);
                    let t_end = f32::INFINITY.into();
                    let origin = self.point;
                    $ray::with(origin, direction, t_start, t_end)
                }
            }
        )+
    };
}

geometry_info!(
    GeometryInfo => Ray, f32, Vec3,
    GeometryInfox4 => Ray4, f32x4, Vec3x4
);

impl MinMaxExt for GeometryInfo {
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

impl PartialEq for GeometryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.point == other.point && self.normal == other.normal
    }
}

impl PartialEq for GeometryInfox4 {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
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

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo>;
    
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

pub struct DefaultGeometry;

impl Geometry for DefaultGeometry {
    fn bounding_box(&self) -> Aabb {
        Aabb::inverted_infinite()
    }

    fn intersect(&self, _: &Ray) -> Option<GeometryInfo> {
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
