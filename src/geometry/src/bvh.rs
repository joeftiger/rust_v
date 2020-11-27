#![allow(dead_code)]

use crate::aabb::Aabb;
use std::sync::Arc;
use crate::{Geometry, GeometryInfo};
use crate::ray::Ray;
use util::{MinMaxExt, floats};

#[derive(Default)]
struct BvhNode {
    aabb: Aabb,
    children: Vec<BvhNode>,
    geometries: Vec<Arc<dyn Geometry>>,
}

impl BvhNode {
    pub fn new(aabb: Aabb,
               children: Vec<BvhNode>,
               geometries: Vec<Arc<dyn Geometry>>) -> Self {
        Self { aabb, children, geometries }
    }

    pub fn add_child(&mut self, node: BvhNode) {
        self.children.push(node)
    }

    pub fn add_geometry(&mut self, geometry: Arc<dyn Geometry>) {
        self.geometries.push(geometry)
    }

    pub fn shrink_to_fit(&mut self) {
        self.children.shrink_to_fit();
        self.geometries.shrink_to_fit()
    }

    pub fn clear(&mut self) {
        self.children.iter_mut().for_each(|n| n.clear());
        self.children.clear();
        self.geometries.clear();
    }
}

impl Geometry for BvhNode {
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        if !self.aabb.intersects(ray) {
            return None;
        }

        let child_intersection = self.children
            .iter()
            .filter_map(|c| c.intersect(ray))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        let geom_intersection = self.geometries
            .iter()
            .filter_map(|g| g.intersect(ray))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        GeometryInfo::mmin_op2(child_intersection, geom_intersection)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        if self.aabb.intersects(ray) {
            let child_hit = self.children
                .iter()
                .any(|c| c.intersects(ray));

            let geom_hit = self.geometries.iter().any(|g| g.intersects(ray));

            child_hit || geom_hit
        } else {
            false
        }
    }
}
