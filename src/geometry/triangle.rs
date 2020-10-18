use std::ops::Index;

use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

use crate::geometry::{Boxable, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::Float;

/// A geometrical triangle.
///
/// The vertices are also accessible through the `Index<usize>` trait.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Triangle {
    pub vertex0: Vec3,
    pub vertex1: Vec3,
    pub vertex2: Vec3,
}

impl Triangle {
    pub fn new(vertex0: Vec3, vertex1: Vec3, vertex2: Vec3) -> Self {
        Self {
            vertex0, vertex1, vertex2,
        }
    }
}

impl Index<usize> for Triangle {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.vertex0,
            1 => &self.vertex1,
            2 => &self.vertex2,
            _ =>  panic!("Index out of range. Valid inputs are in the range of [0, 1, 2]")
        }
    }
}

impl Boxable for Triangle {
    fn bounding_box(&self) -> Option<Aabb> {
        let min = self[0].min_by_component(self[1].min_by_component(self[2]));
        let max = self[0].max_by_component(self[1].max_by_component(self[2]));

        Some(Aabb::new(min, max))
    }
}

impl Intersectable<Ray> for Triangle {
    // According to the Möller–Trumbore intersection algorithm (Wikipedia)
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];

        let h = ray.direction.cross(edge2);

        let a = edge1.dot(h) as Float;
        if a.abs() <= Float::EPSILON {
            return None;
        }

        let f = 1.0 / a as f32;
        let s = ray.origin - self[0];

        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || v > 1.0 {
            return None;
        }

        let t = (f * edge2.dot(q)) as Float;
        if t <= Float::EPSILON {
            return None;
        }

        let triangle_normal = edge1.cross(edge2);

        let position = ray.at(t);
        let mut normal = triangle_normal;
        if normal.dot(ray.direction) < 0.0 {
            normal = -normal;
        }

        Some(Intersection::new(position, normal, t))
    }
}
