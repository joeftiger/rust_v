#![allow(dead_code)]

use crate::{DistanceExt, Geometry, GeometryInfo};
use crate::aabb::Aabb;
use crate::ray::Ray;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use util::floats;

#[derive(Debug)]
pub struct Bvh<T> {
    pub aabb: Aabb,
    pub children: Vec<Arc<Bvh<T>>>,
    pub objects: Vec<Arc<T>>,
}

impl<T> Default for Bvh<T> {
    fn default() -> Self {
        Self::new(Aabb::inverted_infinite(), vec![], vec![])
    }
}

impl<T: PartialEq> PartialEq for Bvh<T> {
    fn eq(&self, other: &Self) -> bool {
        self.aabb == other.aabb && self.children == other.children && self.objects == other.objects
    }
}

impl<T> Bvh<T> {
    pub fn new(aabb: Aabb, children: Vec<Arc<Bvh<T>>>, objects: Vec<Arc<T>>) -> Self {
        Self { aabb, children, objects }
    }
}

impl<T: Geometry> Bvh<T> {
    pub fn build_tree_vec(objects: Vec<Arc<T>>) -> Arc<Self> {
        Self::build_tree(objects.into_iter().enumerate().collect())
    }

    pub fn build_tree(mut objects: HashMap<usize, Arc<T>>) -> Arc<Self> {
        if objects.is_empty() {
            return Arc::new(Self::default());
        } else if objects.len() == 1 {
            let object = objects.drain().next().unwrap();
            let aabb = object.1.bounding_box();

            return Arc::new(Self::new(aabb, vec![], vec![object.1]));
        } else if objects.len() == 2 {
            let mut drain = objects.drain();
            let o1 = drain.next().unwrap();
            let o2 = drain.next().unwrap();
            let aabb = o1.1.bounding_box().outer_join(&o2.1.bounding_box());

            return Arc::new(Self::new(aabb, vec![], vec![o1.1, o2.1]));
        }

        let mut nodes: HashMap<usize, Arc<Self>> = HashMap::default();
        let mut node_counter = 0;

        // create tree by closest bounding box center distances.
        while !objects.is_empty() || nodes.len() > 1 {
            let mut oo = None;
            let mut on = None;
            let mut nn = None;

            let mut distance = f32::INFINITY;

            objects.iter().for_each(|first| {
                objects.iter().for_each(|second| if first.0 != second.0 {
                    let d = first.1.bounding_box().distance(&second.1.bounding_box());
                    if d < distance {
                        distance = d;
                        oo = Some((*first.0, *second.0));
                        on = None;
                        nn = None;
                    }
                });

                nodes.iter_mut().for_each(|second| {
                    let d = first.1.bounding_box().distance(&second.1.bounding_box());
                    if d < distance {
                        distance = d;
                        oo = None;
                        on = Some((*first.0, *second.0));
                        nn = None;
                    }
                })
            });

            nodes.iter().for_each(|first| nodes.iter().for_each(|second| if first.0 != second.0 {
                let d = first.1.bounding_box().distance(&second.1.bounding_box());
                if d < distance {
                    distance = d;
                    oo = None;
                    on = None;
                    nn = Some((*first.0, *second.0));
                }
            }));

            let (children, objects) = if let Some(oo) = oo {
                let o1 = objects.remove(&oo.0).expect("Key was not in objects map anymore");
                let o2 = objects.remove(&oo.1).expect("Key was not in objects map anymore");

                (vec![], vec![o1, o2])
            } else if let Some(on) = on {
                let o = objects.remove(&on.0).expect("Key was not in objects map anymore");
                let n = nodes.remove(&on.1).expect("Key was not in nodes map anymore");

                (vec![n], vec![o])
            } else if let Some(nn) = nn {
                let n1 = nodes.remove(&nn.0).expect("Key was not in nodes map anymore");
                let n2 = nodes.remove(&nn.1).expect("Key was not in nodes map anymore");

                (vec![n1, n2], vec![])
            } else {
                unreachable!()
            };

            let aabb = children
                .iter()
                .map(|c| c.bounding_box())
                .chain(objects.iter().map(|o| o.bounding_box()))
                .fold(Aabb::inverted_infinite(), |acc, next| acc.outer_join(&next));

            let key = node_counter;
            node_counter += 1;

            let new_node = Self::new(aabb, children, objects);
            nodes.insert(key, Arc::new(new_node));
        }

        assert_eq!(nodes.len(), 1);

        let super_node = nodes.drain().next().unwrap();
        super_node.1
    }
}

impl<T: Debug + Geometry + Send + Sync> Geometry for Bvh<T> {
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        if !self.bounding_box().intersects(ray) {
            return None;
        }

        let obj_intersection = self.objects
            .iter()
            .filter_map(|o| o.intersect(ray))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        if let Some(obj) = obj_intersection {
            let mut new_ray = *ray;
            new_ray.t_end = obj.t;

            let child_intersection = self.children
                .iter()
                .filter_map(|c| c.intersect(ray))
                .min_by(|a, b| floats::fast_cmp(a.t, b.t));

            if let Some(child) = child_intersection {
                if child.t < obj.t {
                    return child_intersection;
                }
            }

            obj_intersection
        } else {self.children
            .iter()
            .filter_map(|c| c.intersect(ray))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t))
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.objects.iter().any(|o| o.intersects(ray)) || self.children.iter().any(|c| c.intersects(ray))
    }
}
