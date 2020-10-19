use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

use crate::geometry::plane::Plane;
use crate::geometry::ray::Ray;
use crate::geometry::{Intersectable, Intersection};
use crate::Float;

/// Creates a unit Aabb with size `(-Vec3::one(), Vec3::one())`.
pub fn unit_aabb() -> Aabb {
    Aabb::new(-Vec3::one(), Vec3::one())
}

/// # Summary
/// Optimized ray intersection against a unit Aabb
///
pub fn intersects_unit_aabb(ray: &Ray) -> Option<Intersection> {
    let t1 = (-Vec3::one() - ray.origin) / ray.direction;
    let t2 = (Vec3::one() - ray.origin) / ray.direction;

    let t_min_vec = t1.min_by_component(t2);
    let t_max_vec = t1.max_by_component(t2);

    let t_min = Float::max(
        t_min_vec.z as Float,
        Float::max(t_min_vec.y as Float, t_min_vec.x as Float),
    );
    let t_max = Float::min(
        t_max_vec.z as Float,
        Float::min(t_max_vec.y as Float, t_max_vec.x as Float),
    );

    if t_max < 0.0 || t_max < t_min {
        return None;
    }

    let position = ray.at(t_min);
    let normal: Vec3;

    if (position.x + 1.0) as Float <= Float::EPSILON {
        normal = -Vec3::unit_x();
    } else if (position.x - 1.0) as Float <= Float::EPSILON {
        normal = Vec3::unit_x();
    } else if (position.y + 1.0) as Float <= Float::EPSILON {
        normal = -Vec3::unit_y();
    } else if (position.y - 1.0) as Float <= Float::EPSILON {
        normal = Vec3::unit_y();
    } else if (position.z + 1.0) as Float <= Float::EPSILON {
        normal = -Vec3::unit_z();
    } else {
        normal = Vec3::unit_z();
    }

    Some(Intersection::new(position, normal, t_min))
}

/// An geometrical axis-aligned bounding box.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Aabb {
    /// The minimum position of the aabb.
    pub min: Vec3,
    /// The maximum position of the aabb.
    pub max: Vec3,
}

impl Aabb {
    /// Creates a new aabb.
    ///
    /// It minimizes `min` anx maximizes `max` in case of argument error.
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        let min = min.min_by_component(max);
        let max = max.max_by_component(min);
        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, target: Vec3) -> bool {
        target.x >= self.min.x
            && target.x <= self.max.x
            && target.y >= self.min.y
            && target.y <= self.max.y
            && target.z >= self.min.z
            && target.z <= self.max.z
    }

    /// Creates the inner join / intersection of both aabbs.
    #[inline]
    #[must_use]
    pub fn inner_join(&self, other: &Self) -> Self {
        Self {
            min: self.min.max_by_component(other.min),
            max: self.max.min_by_component(other.max),
        }
    }

    /// Creates the outer join / overlap of both aabbs.
    #[inline]
    #[must_use]
    pub fn outer_join(&self, other: &Self) -> Self {
        Self {
            min: self.min.min_by_component(other.min),
            max: self.max.max_by_component(other.max),
        }
    }

    /// Calculates the volume.
    #[inline]
    #[must_use]
    pub fn volume(&self) -> Float {
        // max is guaranteed to be greater-or-equal to min.
        ((self.max.x - self.min.x) * (self.max.y - self.min.y) * (self.max.z - self.min.z).abs())
            as Float
    }

    /// Calculates the size.
    #[inline]
    #[must_use]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Calculate the center
    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3 {
        (self.max + self.min) / 2.0
    }

    pub fn x_plane_min(&self) -> Plane {
        Plane::new(self.max.x as Float, -Vec3::unit_x())
    }

    pub fn x_plane_max(&self) -> Plane {
        Plane::new(self.max.x as Float, Vec3::unit_x())
    }

    pub fn y_plane_min(&self) -> Plane {
        Plane::new(self.max.y as Float, -Vec3::unit_y())
    }

    pub fn y_plane_max(&self) -> Plane {
        Plane::new(self.max.y as Float, Vec3::unit_y())
    }

    pub fn z_plane_min(&self) -> Plane {
        Plane::new(self.max.z as Float, -Vec3::unit_z())
    }

    pub fn z_plane_max(&self) -> Plane {
        Plane::new(self.max.z as Float, Vec3::unit_z())
    }
}

impl Intersectable<Self> for Aabb {
    #[inline]
    #[must_use]
    fn intersects(&self, other: &Self) -> Option<Intersection> {
        if (self.min.x <= other.max.x && self.max.x >= other.min.x)
            && (self.min.y <= other.max.y && self.max.y >= other.min.y)
            && (self.min.z <= other.max.z && self.max.z >= other.min.z)
        {
            return Some(Intersection::default());
        }

        None
    }
}

impl Intersectable<Ray> for Aabb {
    #[inline]
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = Float::max(
            t_min_vec.z as Float,
            Float::max(t_min_vec.y as Float, t_min_vec.x as Float),
        );
        let t_max = Float::min(
            t_max_vec.z as Float,
            Float::min(t_max_vec.y as Float, t_max_vec.x as Float),
        );

        if t_max < 0.0 || t_max < t_min {
            return None;
        }

        let position = ray.at(t_min);
        let normal: Vec3;

        if (position.x - self.min.x) as Float <= Float::EPSILON {
            normal = -Vec3::unit_x();
        } else if (position.x - self.max.x) as Float <= Float::EPSILON {
            normal = Vec3::unit_x();
        } else if (position.y - self.min.y) as Float <= Float::EPSILON {
            normal = -Vec3::unit_y();
        } else if (position.y - self.max.y) as Float <= Float::EPSILON {
            normal = Vec3::unit_y();
        } else if (position.z - self.min.z) as Float <= Float::EPSILON {
            normal = -Vec3::unit_z();
        } else {
            normal = Vec3::unit_z();
        }

        Some(Intersection::new(position, normal, t_min))
    }
}
