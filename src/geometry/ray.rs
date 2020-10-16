use std::fmt::Debug;

use ultraviolet::vec::Vec3;

use crate::geometry::AngularExt;

#[derive(Copy, Clone, Debug, Default)]
pub struct NormalRay {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub trait Ray {
    #[must_use]
    fn at(&self, t: f32) -> Vec3;

    #[must_use]
    fn distance(&self, p: Vec3) -> f32;

    #[must_use]
    fn origin(&self) -> Vec3;

    #[must_use]
    fn direction(&self) -> Vec3;
}

impl NormalRay {
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

impl Ray for NormalRay {
    #[inline]
    #[must_use]
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    #[inline]
    #[must_use]
    fn distance(&self, p: Vec3) -> f32 {
        (p - self.origin).mag()
    }

    fn origin(&self) -> Vec3 {
        self.origin
    }

    fn direction(&self) -> Vec3 {
        self.direction
    }
}

impl AngularExt<Self> for NormalRay {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Self) -> f32 {
        self.direction.angle_to(other.direction)
    }
}

impl AngularExt<Vec3> for NormalRay {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Vec3) -> f32 {
        self.direction.angle_to(other)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ShadowRay {
    pub ray: NormalRay,
    pub max_length: f32,
}

impl ShadowRay {
    pub fn new(ray: NormalRay, max_length: f32) -> Self {
        Self {
            ray,
            max_length
        }
    }

    pub fn in_range(&self, t: f32) -> bool {
        t <= self.max_length
    }

    #[inline]
    #[must_use]
    pub fn at(&self, t: f32) -> Vec3 {
        self.ray.at(t)
    }

    #[inline]
    #[must_use]
    pub fn distance(&self, p: Vec3) -> f32 {
        self.ray.distance(p)
    }
}

impl Ray for ShadowRay {
    #[inline]
    #[must_use]
    fn at(&self, t: f32) -> Vec3 {
        self.ray.at(t)
    }

    #[inline]
    #[must_use]
    fn distance(&self, p: Vec3) -> f32 {
        self.ray.distance(p)
    }

    fn origin(&self) -> Vec3 {
        self.ray.origin()
    }

    fn direction(&self) -> Vec3 {
        self.ray.direction()
    }
}

impl AngularExt<Self> for ShadowRay {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Self) -> f32 {
        self.ray.angle_to(other.ray)
    }
}

impl AngularExt<NormalRay> for ShadowRay {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: NormalRay) -> f32 {
        self.ray.angle_to(other)
    }
}

impl AngularExt<Vec3> for ShadowRay {
    #[inline]
    #[must_use]
    fn angle_to(&self, other: Vec3) -> f32 {
        self.ray.angle_to(other)
    }
}
