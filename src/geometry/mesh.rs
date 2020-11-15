use crate::floats;
use crate::formats::obj::ObjFile;
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo};
use std::ops::Deref;
use std::rc::Rc;
use ultraviolet::Vec3;

pub struct Triangle {
    a: Rc<Vec3>,
    b: Rc<Vec3>,
    c: Rc<Vec3>,
}

impl Triangle {
    pub fn new(a: Rc<Vec3>, b: Rc<Vec3>, c: Rc<Vec3>) -> Self {
        Self { a, b, c }
    }
}

impl Geometry for Triangle {
    fn bounding_box(&self) -> Aabb {
        let min = self.a.min_by_component(self.b.min_by_component(*self.c));
        let max = self.a.max_by_component(self.b.max_by_component(*self.c));

        Aabb::new(min, max)
    }

    #[allow(clippy::many_single_char_names)]
    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let ab = *self.b - *self.a;
        let ac = *self.c - *self.a;
        let h = ray.direction.cross(ac);

        let det = ab.dot(h);
        if det < floats::EPSILON {
            return None;
        }

        let t = ray.origin - *self.a;
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
        if t < ray.t_start || ray.t_end < t {
            return None;
        }

        let point = ray.at(t);

        let ab = *self.b - *self.a;
        let ac = *self.c - *self.a;
        let mut normal = ac.cross(ab);

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(GeometryInfo::new(*ray, t, point, normal))
    }
}

#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Rc<Vec3>>,
    triangles: Vec<Triangle>,
    aabb: Aabb,
}

impl Mesh {
    pub fn new(vertices: Vec<Rc<Vec3>>, triangles: Vec<Triangle>) -> Self {
        let mut aabb = Aabb::inverted_infinite();
        vertices.iter().for_each(|v| {
            aabb.min = aabb.min.min_by_component(*v.deref());
            aabb.max = aabb.max.max_by_component(*v.deref());
        });

        debug_assert!(aabb.is_valid());

        Self {
            vertices,
            triangles,
            aabb,
        }
    }
}

impl Geometry for Mesh {
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let mut current_info: Option<GeometryInfo> = None;
        self.triangles.iter().for_each(|t: &Triangle| {
            if let Some(i) = t.intersect(ray) {
                if current_info.is_none() {
                    current_info = Some(i);
                    return;
                }
                if i.t < current_info.unwrap().t {
                    current_info = Some(i);
                }
            }
        });

        current_info
    }
}

impl From<ObjFile> for Mesh {
    fn from(file: ObjFile) -> Self {
        debug_assert!({
            file.assert_ok();
            true
        });

        let mut mesh = Mesh::default();
        file.v.iter().for_each(|v| {
            let vertex = Vec3::new(v.0, v.1, v.2);
            mesh.vertices.push(Rc::new(vertex));
        });
        file.f.iter().for_each(|f| {
            let v = f.v;
            // off by one due to .obj counting
            let a = mesh.vertices[v.0 - 1].clone();
            let b = mesh.vertices[v.1 - 1].clone();
            let c = mesh.vertices[v.2 - 1].clone();

            let triangle = Triangle::new(a, b, c);
            mesh.triangles.push(triangle);
        });

        mesh
    }
}
