#![allow(dead_code)]

use std::sync::Arc;

use geometry::{DistanceExt, Geometry, GeometryInfo};
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use util::floats;

use crate::render::scene::SceneIntersection;
use crate::render::scene_objects::SceneObject;

#[derive(Debug)]
enum TypeChecks {
    Geometry,
    Node,
}

pub fn build_tree(mut scene_objects: Vec<Arc<SceneObject>>) -> Arc<BvhNode> {
    if scene_objects.is_empty() {
        return Arc::new(BvhNode::new_empty());
    } else if scene_objects.len() == 1 {
        let object = scene_objects.pop().unwrap();
        return Arc::new(BvhNode::new(object.bounding_box(), vec![], vec![object]));
    }

    let mut nodes: Vec<Arc<BvhNode>> = Vec::default();

    // create tree by closest bounding box center distances.
    while !scene_objects.is_empty() || nodes.len() > 1 {
        let mut best = None;
        let mut distance = f32::INFINITY;
        let mut type_0 = TypeChecks::Geometry;
        let mut type_1 = TypeChecks::Geometry;

        // check best / closest
        for i in 0..scene_objects.len() {
            // geometry - geometry
            for j in (i + 1)..scene_objects.len() {
                let d = scene_objects[i].bounding_box().distance(&scene_objects[j].bounding_box());
                if d < distance {
                    distance = d;
                    best = Some((i, j));
                }
            }

            // geometry - node
            #[allow(clippy::needless_range_loop)]
            for j in 0..nodes.len() {
                let d = scene_objects[i].bounding_box().distance(&nodes[j].bounding_box());
                if d < distance {
                    distance = d;
                    best = Some((i, j));
                    type_1 = TypeChecks::Node;
                }
            }
        }
        // node - node
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
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
            let (children, objects) = match type_0 {
                TypeChecks::Geometry => match type_1 {
                    TypeChecks::Geometry => {
                        let objects = vec![scene_objects[best.0].clone(), scene_objects[best.1].clone()];
                        scene_objects.swap_remove(best.0.max(best.1));
                        scene_objects.swap_remove(best.0.min(best.1));
                        (vec![], objects)
                    },
                    TypeChecks::Node => (vec![nodes.swap_remove(best.1)], vec![scene_objects.swap_remove(best.0)])
                },
                TypeChecks::Node => match type_1 {
                    TypeChecks::Geometry => (vec![nodes.swap_remove(best.0)], vec![scene_objects.swap_remove(best.1)]),
                    TypeChecks::Node => {
                        let children = vec![nodes[best.0].clone(), nodes[best.1].clone()];
                        nodes.swap_remove(best.0.max(best.1));
                        nodes.swap_remove(best.0.min(best.1));
                        (children, vec![])
                    }
                },
            };

            let aabb = children
                .iter()
                .map(|c| c.bounding_box())
                .chain(objects.iter().map(|o| o.bounding_box()))
                .fold(Aabb::inverted_infinite(), |acc, next| acc.outer_join(&next));

            let new_node = BvhNode::new(aabb, children, objects);
            nodes.push(Arc::new(new_node));
        }
    }

    assert_eq!(nodes.len(), 1);

    nodes.pop().unwrap()
}


#[derive(Default)]
pub struct BvhNode {
    aabb: Aabb,
    children: Vec<Arc<BvhNode>>,
    objects: Vec<Arc<SceneObject>>,
}

impl BvhNode {
    pub fn new_empty() -> Self {
        let aabb = Aabb::inverted_infinite();

        Self::new(aabb, vec![], vec![])
    }

    pub fn new(aabb: Aabb,
               children: Vec<Arc<BvhNode>>,
               objects: Vec<Arc<SceneObject>>) -> Self {
        Self { aabb, children, objects }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        if !self.aabb.intersects(ray) {
            return None;
        }

        // object intersections
        let object_intersection = self.objects.iter().filter_map(|n| if let Some(info) = n.intersect(&ray) {
            Some(SceneIntersection::new(info, n.clone()))
        } else {
            None
        })
            .min_by(|a, b| floats::fast_cmp(a.info.t, b.info.t));

        // child intersections
        if let Some(object_intersection) = object_intersection {
            let mut new_ray = *ray;
            new_ray.t_end = object_intersection.info.t;

            let closest_child = self.children
                .iter()
                .filter_map(|n| n.intersect(&new_ray))
                .min_by(|a, b| floats::fast_cmp(a.info.t, b.info.t));

            if let Some(closest) = closest_child {
                if closest.info.t < object_intersection.info.t {
                    return Some(closest);
                }
            }

            Some(object_intersection)
        } else {
            self.children
                .iter()
                .filter_map(|n| n.intersect(&ray))
                .min_by(|a, b| floats::fast_cmp(a.info.t, b.info.t))
        }
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

        self.intersect(ray).map(|si| si.info)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        if self.aabb.intersects(ray) {
            self.children.iter().any(|c| c.intersects(ray)) || self.objects.iter().any(|o| o.intersects(ray))
        } else {
            false
        }
    }
}
