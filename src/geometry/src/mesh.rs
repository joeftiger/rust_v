use std::sync::Arc;

use tobj::Mesh as TobjMesh;
use ultraviolet::{Rotor3, Vec3};
use util::floats;

use crate::aabb::Aabb;
use crate::bvh::Bvh;
use crate::ray::Ray;
use crate::{Intersection, Boundable, Intersectable};

#[derive(Debug, PartialEq)]
pub struct Triangle {
    a: Arc<Vec3>,
    b: Arc<Vec3>,
    c: Arc<Vec3>,
}

impl Triangle {
    pub fn new(a: Arc<Vec3>, b: Arc<Vec3>, c: Arc<Vec3>) -> Self {
        Self { a, b, c }
    }
}

impl Boundable for Triangle {
    fn bounds(&self) -> Aabb {
        let min = self.a.min_by_component(self.b.min_by_component(*self.c));
        let max = self.a.max_by_component(self.b.max_by_component(*self.c));

        Aabb::new(min, max)
    }
}

impl Intersectable for Triangle {
    #[allow(clippy::many_single_char_names)]
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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
        let mut normal = ac.cross(ab).normalized();

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(Intersection::new(*ray, t, point, normal))
    }

    #[allow(clippy::many_single_char_names)]
    fn intersects(&self, ray: &Ray) -> bool {
        let ab = *self.b - *self.a;
        let ac = *self.c - *self.a;
        let h = ray.direction.cross(ac);

        let det = ab.dot(h);
        if det < floats::EPSILON {
            return false;
        }

        let t = ray.origin - *self.a;
        let u = t.dot(h) / det;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = t.cross(ab);
        let v = ray.direction.dot(q) / det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = ac.dot(q) / det;

        ray.is_in_range(t)
    }
}

// impl Geometry for Triangle {
//     fn surface_area(&self) -> f32 {
//         let b_vec = *self.a - *self.c;
//         let c_vec = *self.b - *self.a;
//         let alpha = b_vec.angle_to(&c_vec);
//
//         b_vec.mag() * c_vec.mag() * alpha.sin() / 2.0
//     }
//
//     fn sample_surface(&self, sample: &Vec3) -> Vec3 {
//         let x_sqrt = sample.x.sqrt();
//         let a = *self.a * (1.0 - x_sqrt);
//         let b = *self.b * (x_sqrt * (1.0 - sample.y));
//         let c = *self.c * (sample.y * x_sqrt);
//
//         a + b + c
//     }
// }

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq)]
pub struct Mesh {
    vertices: Vec<Arc<Vec3>>,
    triangles: Vec<Arc<Triangle>>,
    bvh: Arc<Bvh<Triangle>>,
}

impl Mesh {
    pub fn new(vertices: Vec<Arc<Vec3>>, triangles: Vec<Arc<Triangle>>) -> Self {
        let bvh = Bvh::aac_vec(triangles.clone());

        Self {
            vertices,
            triangles,
            bvh,
        }
    }

    pub fn load_scale_floor_rot(
        (tobj_mesh, scale, center_floor, rotation): (&TobjMesh, Vec3, Vec3, Rotor3),
    ) -> Self {
        let mut vertices = Vec::with_capacity(tobj_mesh.positions.len());
        let mut v_center = Vec3::zero();
        let mut minimum_y = f32::INFINITY;

        let mut i = 0;
        while i < tobj_mesh.positions.len() {
            let vertex = Vec3::new(
                tobj_mesh.positions[i] * scale.x,
                tobj_mesh.positions[i + 1] * scale.y,
                tobj_mesh.positions[i + 2] * scale.z,
            );
            vertices.push(vertex);
            v_center += vertex;
            minimum_y = minimum_y.min(vertex.y);
            i += 3;
        }
        let vertices: Vec<Arc<Vec3>> = vertices
            .iter()
            .map(|v| {
                let mut v = *v + center_floor;
                v.y -= minimum_y;

                let mut v_tmp = v - center_floor;
                v_tmp = rotation * v_tmp;
                v = v_tmp + center_floor;

                Arc::new(v)
            })
            .collect();

        let mut triangles = Vec::with_capacity(tobj_mesh.indices.len() / 3);
        let mut i = 0;
        while i < tobj_mesh.indices.len() {
            let a = vertices[tobj_mesh.indices[i] as usize].clone();
            let b = vertices[tobj_mesh.indices[i + 1] as usize].clone();
            let c = vertices[tobj_mesh.indices[i + 2] as usize].clone();

            let triangle = Triangle::new(a, b, c);
            triangles.push(Arc::new(triangle));
            i += 3;
        }
        triangles.shrink_to_fit();

        Mesh::new(vertices, triangles)
    }
}

impl Boundable for Mesh {
    fn bounds(&self) -> Aabb {
        self.bvh.bounds()
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.bvh.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.bvh.intersects(ray)
    }
}

// impl Geometry for Mesh {
//
//     fn surface_area(&self) -> f32 {
//         0.0
//     }
//
//     fn sample_surface(&self, sample: &Vec3) -> Vec3 {
//         // triangle sampling uses only sample.xy, z is the remaining random variable
//         let index = (sample.z * self.triangles.len() as f32) as usize;
//
//         self.triangles[index].sample_surface(sample)
//     }
// }
