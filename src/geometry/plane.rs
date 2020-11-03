use ultraviolet::Vec3;
use crate::geometry::{AngularExt, Container, GeometryInfo};
use crate::geometry::ray::Ray;
use crate::floats;
use crate::util::MinMaxExt;

#[derive(Debug, PartialEq)]
pub struct Plane {
    pub normal: Vec3,
    /// The distance of the plane into the normal direction
    pub d: f32,
}

#[allow(dead_code)]
impl Plane {
    pub fn new(normal: Vec3, d: f32) -> Self {
        Self { normal, d }
    }

    /// Calculates the distance of the given point to the plane.
    #[inline]
    #[must_use]
    pub fn distance(&self, point: Vec3) -> f32 {
        let v = point - (self.normal * self.d);
        let angle = self.normal.angle_to(&v);

        angle.cos() * v.mag()
    }

    /// Calculates the projected position of the given point to the plane.
    #[inline]
    #[must_use]
    pub fn project(&self, point: Vec3) -> Vec3 {
        point - self.normal * self.distance(point)
    }

    pub fn from(p: &Plane2) -> (Plane, Plane) {
        (Self::new(p.normal, p.d0), Self::new(-p.normal, p.d0))
    }

    pub fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let denom = self.normal.dot(ray.direction);
        if denom < floats::DEFAULT_EPSILON {
            return None;
        }

        let p = self.normal * self.d - ray.origin;
        let t = p.dot(self.normal) / denom;
        if t < 0.0 || t < ray.t {
            return None;
        }

        let point = ray.at(t);

        Some(GeometryInfo::new(*ray, t, point, self.normal))
    }
}

#[derive(Debug, PartialEq)]
pub struct Plane2 {
    pub normal: Vec3,
    /// The distance of the plane into the normal direction
    pub d0: f32,
    pub d1: f32,
}

impl Plane2 {
    pub fn new(normal: Vec3, d0: f32, d1: f32) -> Self {
        Self { normal, d0, d1 }
    }

    /// Calculates the distance of the given point to the planes.
    #[inline]
    #[must_use]
    pub fn distance(&self, point: Vec3) -> (f32, f32) {
        let v0 = point - (self.normal * self.d0);
        let v1 = point - (self.normal * self.d1);
        let angle = self.normal.angle_to(&v0);

        let cos = angle.cos();

        (cos * v0.mag(), cos * v1.mag())
    }

    /// Calculates the projected position of the given point to the planes.
    #[inline]
    #[must_use]
    pub fn project(&self, point: Vec3) -> (Vec3, Vec3) {
        let (d0, d1) = self.distance(point);
        let p0 = point - self.normal * d0;
        let p1 = point - self.normal * d1;

        (p0, p1)
    }

    /// the height between the planes
    #[inline]
    #[must_use]
    pub fn height(&self) -> f32 {
        f32::abs(self.d1 - self.d0)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let (p0, p1) = Plane::from(self);

        GeometryInfo::mmin_op2(p0.intersect(ray), p1.intersect(ray))
    }
}

impl Container for Plane2 {
    fn contains(&self, obj: Vec3) -> bool {
        let diff = f32::abs(self.d1 - self.d0);
        let (d0, d1) = self.distance(obj);

        (d0 <= diff) && (d1 <= diff)
    }
}
