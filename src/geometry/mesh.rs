use crate::floats;
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo};
use ultraviolet::Vec3;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }
}

impl Geometry for Triangle {
    fn bounding_box(&self) -> Aabb {
        let min = self.a.min_by_component(self.b.min_by_component(self.c));
        let max = self.a.max_by_component(self.b.max_by_component(self.c));

        Aabb::new(min, max)
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let h = ray.direction.cross(ac);

        let det = ab.dot(h);
        if det < floats::DEFAULT_EPSILON {
            return None;
        }

        let t = ray.origin - self.a;
        let u = t.dot(h) / det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = t.cross(ab);
        let v = ray.direction.dot(q) / det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = ac.dot(q) / det;
        if ray.t < t {
            return None;
        }

        let point = ray.at(t);

        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let normal = ac.cross(ab);

        Some(GeometryInfo::new(*ray, t, point, normal))
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    aabb: Aabb,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>) -> Self {
        let mut aabb = Aabb::inverted_infinite();
        vertices.iter().for_each(|v| {
            aabb.min = aabb.min.min_by_component(*v);
            aabb.max = aabb.max.max_by_component(*v);
        });

        debug_assert!(aabb.is_valid());

        Self {
            vertices,
            indices,
            aabb,
        }
    }
}

impl Geometry for Mesh {
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    #[allow(unused_variables)]
    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        unimplemented!()
    }
}
