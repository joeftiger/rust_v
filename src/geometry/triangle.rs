use std::ops::Index;

use ultraviolet::Vec3;

use crate::geometry::{Boxable, Intersectable, Intersection, Ray};
use crate::geometry::aabb::Aabb;

/// A triangle consists of 3 vertices.
///
/// They are accessible through the `Index<usize>` trait.
pub struct Triangle {
    vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(vertex0: Vec3, vertex1: Vec3, vertex2: Vec3) -> Self {
        Self {
            vertices: [vertex0, vertex1, vertex2],
        }
    }
}

impl Index<usize> for Triangle {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl Boxable for Triangle {
    fn bounding_box(&self) -> Option<Aabb> {
        let min = self[0].min_by_component(self[1].min_by_component(self[2]));
        let max = self[0].max_by_component(self[1].max_by_component(self[2]));

        Some(Aabb::new(min, max))
    }
}

impl Intersectable<&Ray> for Triangle {
    // According to the Möller–Trumbore intersection algorithm (Wikipedia)
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];

        let h = ray.direction.cross(edge2);

        let a = edge1.dot(h);
        if a.abs() <= f32::EPSILON {
            return None;
        }

        let f = 1.0 / a;
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

        let t = f * edge2.dot(q);
        if t <= f32::EPSILON {
            return None;
        }

        let triangle_normal = edge1.cross(edge2);

        let position = ray.at(t);
        let mut normal = triangle_normal;
        if normal.dot(ray.direction) < 0.0 {
            normal = -normal;
        }

        Some(Intersection::new(position, normal))
    }
}
