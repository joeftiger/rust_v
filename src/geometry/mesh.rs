use ultraviolet::Vec3;
use serde::{Deserialize, Serialize};

use crate::geometry::aabb::Aabb;
use crate::geometry::{Boxable, Intersectable, Intersection};
use std::ops::IndexMut;
use crate::geometry::ray::Ray;


#[derive(Debug, Deserialize, Serialize)]
pub enum DrawMode {
    FLAT,
    PHONG,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Vertex {
    position: Vec3,
    normal: Vec3,
}

#[derive(Debug, Deserialize, Serialize)]
struct Triangle {
    index0: usize,
    index1: usize,
    index2: usize,
    normal: Vec3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mesh {
    draw_mode: DrawMode,
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    aabb: Aabb,
}

impl Mesh {
    pub fn compute_normals(&mut self) {
        // compute triangle normals
        for i in 0..self.triangles.len() {
            let triangle = self.triangles.index_mut(i);

            let p0 = self.vertices[triangle.index0].position;
            let p1 = self.vertices[triangle.index1].position;
            let p2 = self.vertices[triangle.index2].position;

            triangle.normal = (p1 - p0).cross(p2 - p0).normalized();
        }

        // initialize vertex normals to zero
        for i in 0..self.vertices.len() {
            let vertex = self.vertices.index_mut(i);
            vertex.normal = Vec3::zero();
        }

        // compute triangle normals and add them to vertices
        for triangle in &self.triangles {
            let (w0, w1, w2) = Mesh::angle_weights(
                self.vertices[triangle.index0].position,
                self.vertices[triangle.index1].position,
                self.vertices[triangle.index2].position,
            );

            // scatter face normals to vertex normals
            self.vertices.index_mut(triangle.index0).normal += w0 * triangle.normal;
            self.vertices.index_mut(triangle.index1).normal += w1 * triangle.normal;
            self.vertices.index_mut(triangle.index2).normal += w2 * triangle.normal;
        }


        // normalize vertex normals
        for i in 0..self.vertices.len() {
            self.vertices.index_mut(i).normal.normalize();
        }
    }

    pub fn compute_bounding_box(&mut self) {
        let mut min = Vec3::one() * f32::INFINITY;
        let mut max = Vec3::one() * f32::NEG_INFINITY;

        for vertex in &self.vertices {
            min = min.min_by_component(vertex.position);
            max = max.max_by_component(vertex.position);
        }

        self.aabb = Aabb::new(min, max);
    }

    /// # Summary
    /// Determines the weights by which to scale triangles (p0, p1, p2)'s normals, when accumulating
    /// the vertex normals for vertices 0, 1 and 2.
    /// (Recall, vertex normals are a weighted average of their incident triangles' normals, and in
    /// this raytracer we'll use the incident angles as weights.)
    ///
    /// # Parameters
    /// p0, p1, p2 ---    triangle vertex positions
    ///
    /// # Returns
    /// weights to be used for vertices 0, 1 and 2
    fn angle_weights(p0: Vec3, p1: Vec3, p2: Vec3) -> (f32, f32, f32) {
        let e01 = (p1 - p0).normalized();
        let e12 = (p2 - p1).normalized();
        let e20 = (p0 - p2).normalized();

        let w0 = f32::max(-1.0, f32::min(1.0, e01.dot(-e20))).acos();
        let w1 = f32::max(-1.0, f32::min(1.0, e12.dot(-e01))).acos();
        let w2 = f32::max(-1.0, f32::min(1.0, e20.dot(-e12))).acos();

        (w0, w1, w2)
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            draw_mode: DrawMode::FLAT,
            vertices: Vec::new(),
            triangles: Vec::new(),
            aabb: Aabb::default(),
        }
    }
}

impl Boxable for Mesh {
    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.aabb)
    }
}

// TODO: IMPLEMENT
impl<T: Ray> Intersectable<T> for Mesh {
    fn intersects(&self, ray: T) -> Option<Intersection> {
        None
    }
}
