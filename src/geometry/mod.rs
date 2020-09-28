use std::ops::Mul;

use ultraviolet::Vec3;

pub mod plane;
pub mod sphere;

pub trait Angular<T> {
    /// Returns the angle in radians.
    #[must_use]
    fn angle_to(&self, other: T) -> f32;
}

pub trait CeilFloorExt {
    fn ceil(&self) -> Self;
    fn floor(&self) -> Self;
}

pub trait Intersectable<T> {
    #[must_use]
    fn intersects(&self, intersector: T) -> bool;
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

#[derive(Debug, Default)]
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

    pub fn to_line(&self) -> Line {
        Line {
            position: self.origin,
            direction: self.direction,
        }
    }
}

impl Intersectable<Vec3> for Ray {
    #[inline]
    #[must_use]
    fn intersects(&self, p: Vec3) -> bool {
        self.direction.angle_to(p + self.origin) <= f32::EPSILON
    }
}

pub struct Line {
    pub position: Vec3,
    pub direction: Vec3,
}

impl Line {
    pub fn new(position: Vec3, direction: Vec3) -> Line {
        Self {
            position,
            direction: direction.normalized(),
        }
    }
}

impl Intersectable<Vec3> for Line {
    fn intersects(&self, p: Vec3) -> bool {
        self.direction.angle_to(p + self.position) <= f32::EPSILON
    }
}

/// An axis-aligned bounding box
#[derive(Debug, Default)]
#[repr(C)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
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

    #[inline]
    #[must_use]
    pub fn volume(&self) -> f32 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y) * (self.max.z - self.min.z)
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> Vec3 {
        self.min.abs() + self.max.abs()
    }
}

impl Intersectable<&Self> for Aabb {
    #[inline]
    #[must_use]
    fn intersects(&self, other: &Self) -> bool {
        (self.min.x <= other.max.x && self.max.x >= other.min.x)
            && (self.min.y <= other.max.y && self.max.y >= other.min.y)
            && (self.min.z <= other.max.z && self.max.z >= other.min.z)
    }
}

impl Intersectable<&Line> for Aabb {
    fn intersects(&self, line: &Line) -> bool {
        let t1 = (self.min - line.position) / line.direction;
        let t2 = (self.max - line.position) / line.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        t_max >= t_min
    }
}

impl Intersectable<&Ray> for Aabb {
    #[inline]
    fn intersects(&self, ray: &Ray) -> bool {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        t_max >= 0.0 && t_max >= t_min
    }
}

impl Mul<f32> for Ray {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.at(rhs)
    }
}
