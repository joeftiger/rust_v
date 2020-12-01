#![allow(dead_code)]

use std::sync::Arc;

use util::floats;

use crate::{Geometry, GeometryInfo, DistanceExt};
use crate::aabb::Aabb;
use crate::ray::Ray;

enum TypeChecks {
    Geometry, Node,
}

pub fn build_tree<T: 'static +  Geometry>(mut geometries: Vec<Arc<T>>) -> Arc<BvhNode> {
    if geometries.is_empty() {
        return Arc::new(BvhNode::new_empty());
    } else if geometries.len() == 1 {
        let geom = geometries.pop().unwrap();
        return Arc::new(BvhNode::new(geom.bounding_box(), vec![geom]));
    }

    let mut nodes: Vec<Arc<BvhNode>> = Vec::default();

    // create tree by closest bounding box center distances.
    while !geometries.is_empty() && nodes.len() > 1 {
        let mut best = None;
        let mut distance = f32::INFINITY;
        let mut type_0 = TypeChecks::Geometry;
        let mut type_1 = TypeChecks::Geometry;

        // check best / closest
        for i in 0..geometries.len() {
            // geometry - geometry
            for j in (i + 1)..geometries.len() {
                let d = geometries[i].bounding_box().distance(&geometries[j].bounding_box());
                if d < distance {
                    distance = d;
                    best = Some((i, j));
                }
            }

            // geometry - node
            for j in 0..nodes.len() {
                let d = geometries[i].bounding_box().distance(&nodes[j].bounding_box());
                if d < distance {
                    distance = d;
                    best = Some((i, j));
                    type_1 = TypeChecks::Node;
                }
            }
        }
        // node - node
        for i in 0..nodes.len() {
            for j in (i +1)..nodes.len() {
                let d = nodes[i].bounding_box().distance(&nodes[j].bounding_box());
                if d < distance {
                    distance = d;
                    best = Some((i, j));
                    type_0 = TypeChecks::Node;
                    type_1 = TypeChecks::Node;
                }
            }
        }

        if let Some(best) = best {
            let a: Arc<dyn Geometry> = match type_0 {
                TypeChecks::Geometry => geometries.swap_remove(best.0),
                TypeChecks::Node => nodes.swap_remove(best.0),
            };
            let b: Arc<dyn Geometry> = match type_1 {
                TypeChecks::Geometry => geometries.swap_remove(best.1),
                TypeChecks::Node => nodes.swap_remove(best.1),
            };

            let aabb = a.bounding_box().outer_join(&b.bounding_box());

            let new_node = BvhNode::new(aabb, vec![a, b]);
            nodes.push(Arc::new(new_node));
        }
    }

    assert_eq!(nodes.len(), 1);

    nodes.pop().unwrap()
}


#[derive(Default)]
pub struct BvhNode {
    aabb: Aabb,
    geometries: Vec<Arc<dyn Geometry>>,
}

impl BvhNode {
    pub fn new_empty() -> Self {
        let aabb = Aabb::inverted_infinite();
        let geometries = vec![];

        Self::new(aabb, geometries)
    }

    pub fn new(aabb: Aabb,
               geometries: Vec<Arc<dyn Geometry>>) -> Self {
        Self { aabb, geometries }
    }

    pub fn shrink_to_fit(&mut self) {
        self.geometries.shrink_to_fit()
    }

    pub fn clear(&mut self) {
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

        self.geometries
            .iter()
            .filter_map(|g| g.intersect(ray))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t))
    }

    fn intersects(&self, ray: &Ray) -> bool {
        if self.aabb.intersects(ray) {
            self.geometries.iter().any(|g| g.intersects(ray))
        } else {
            false
        }
    }
}
