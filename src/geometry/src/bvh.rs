#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use util::floats;

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::{DistanceExt, Intersection, Boundable, Intersectable, Geometry};

#[derive(Debug)]
pub enum BVHNode<T> {
    Leaf(Arc<T>),
    Node {
        left_child: Arc<BVHNode<T>>,
        right_child: Arc<BVHNode<T>>,
        aabb: Aabb,
    },
}

#[derive(Debug)]
pub struct Bvh<T> {
    pub aabb: Aabb,
    pub children: Vec<Arc<Bvh<T>>>,
    pub objects: Vec<T>,
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
    pub fn new(aabb: Aabb, children: Vec<Arc<Bvh<T>>>, objects: Vec<T>) -> Self {
        Self {
            aabb,
            children,
            objects,
        }
    }
}

impl<T> Bvh<T> where T: Boundable {
    pub fn aac_vec(objects: Vec<T>) -> Arc<Self> {
        /*
        println!("# Objects: {}\n", objects.len());

        let mut distances = Vec::with_capacity(objects.len() * objects.len() / 2);

        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                let aabb_0 = objects[i].bounding_box();
                let aabb_1 = objects[j].bounding_box();
                let distance = aabb_0.distance(&aabb_1);

                distances.push(((i, j), distance));
            }
        }
        println!("Created distance matrix\n");

        let mut clusters = Vec::with_capacity((distances.len() + 1) / 2);

        distances.sort_by(|a, b| floats::fast_cmp(a.1, b.1));

        while !distances.is_empty() {
            let ((i, j), _) = distances[0];
            let aabb_0 = objects[i].bounding_box();
            let aabb_1 = objects[j].bounding_box();
            let aabb = aabb_0.outer_join(&aabb_1);

            let leaf_0 = BVHNode::Leaf(objects[i].clone());
            let leaf_1 = BVHNode::Leaf(objects[j].clone());

            let node = BVHNode::Node {
                left_child: Arc::new(leaf_0),
                right_child: Arc::new(leaf_1),
                aabb
            };
            clusters.push(node);

            distances.retain(|(index, _)| index.0 != i && index.0 != j && index.1 != i && index.1 != j);
        }

        println!("Created initial clusters\n");

        let mut distances = Vec::with_capacity(clusters.len() * clusters.len() / 2);

        while clusters.len() > 1 {
            println!("{} clusters left\n", clusters.len());

            for i in 0..clusters.len() {
                for j in (i + 1)..clusters.len() {
                    let aabb_0 = match &clusters[i] {
                        BVHNode::Node { aabb, .. } => aabb,
                        _ => unreachable!(),
                    };
                    let aabb_1 = match &clusters[j] {
                        BVHNode::Node { aabb, .. } => aabb,
                        _ => unreachable!(),
                    };
                    let distance = aabb_0.distance(aabb_1);

                    distances.push(((i, j), distance));
                }
            }
            distances.sort_by(|a, b| floats::fast_cmp(a.1, b.1));

            let ((i, j), _) = distances[0];

            let aabb_0 = match &clusters[i] {
                BVHNode::Node { aabb, .. } => aabb,
                _ => unreachable!(),
            };
            let aabb_1 = match &clusters[j] {
                BVHNode::Node { aabb, .. } => aabb,
                _ => unreachable!(),
            };
            let aabb = aabb_0.outer_join(aabb_1);

            // j > i is guaranteed
            let node = BVHNode::Node {
                left_child: Arc::new(clusters.swap_remove(j)),
                right_child: Arc::new(clusters.swap_remove(i)),
                aabb
            };

            clusters.push(node);

            distances.retain(|(index, _)| index.0 != i && index.0 != j && index.1 != i && index.1 != j);
        }

        println!("{:#?}\n", clusters[0]);
        */

        Self::aac(objects.into_iter().enumerate().collect())
    }

    pub fn aac(mut objects: HashMap<usize, T>) -> Arc<Self> {
        if objects.is_empty() {
            return Arc::new(Self::default());
        } else if objects.len() == 1 {
            let object = objects.drain().next().unwrap();
            let aabb = object.1.bounds();

            return Arc::new(Self::new(aabb, vec![], vec![object.1]));
        } else if objects.len() == 2 {
            let mut drain = objects.drain();
            let o1 = drain.next().unwrap();
            let o2 = drain.next().unwrap();
            let aabb = o1.1.bounds().outer_join(&o2.1.bounds());

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
                objects.iter().for_each(|second| {
                    if first.0 != second.0 {
                        let d = first.1.bounds().distance(&second.1.bounds());
                        if d < distance {
                            distance = d;
                            oo = Some((*first.0, *second.0));
                            on = None;
                            nn = None;
                        }
                    }
                });

                nodes.iter_mut().for_each(|second| {
                    let d = first.1.bounds().distance(&second.1.bounds());
                    if d < distance {
                        distance = d;
                        oo = None;
                        on = Some((*first.0, *second.0));
                        nn = None;
                    }
                })
            });

            nodes.iter().for_each(|first| {
                nodes.iter().for_each(|second| {
                    if first.0 != second.0 {
                        let d = first.1.bounds().distance(&second.1.bounds());
                        if d < distance {
                            distance = d;
                            oo = None;
                            on = None;
                            nn = Some((*first.0, *second.0));
                        }
                    }
                })
            });

            let (children, objects) = if let Some(oo) = oo {
                let o1 = objects
                    .remove(&oo.0)
                    .expect("Key was not in objects map anymore");
                let o2 = objects
                    .remove(&oo.1)
                    .expect("Key was not in objects map anymore");

                (vec![], vec![o1, o2])
            } else if let Some(on) = on {
                let o = objects
                    .remove(&on.0)
                    .expect("Key was not in objects map anymore");
                let n = nodes
                    .remove(&on.1)
                    .expect("Key was not in nodes map anymore");

                (vec![n], vec![o])
            } else if let Some(nn) = nn {
                let n1 = nodes
                    .remove(&nn.0)
                    .expect("Key was not in nodes map anymore");
                let n2 = nodes
                    .remove(&nn.1)
                    .expect("Key was not in nodes map anymore");

                (vec![n1, n2], vec![])
            } else {
                unreachable!("Unreachable. Is an aabb infinite?");
            };

            let aabb = children
                .iter()
                .map(|c| c.bounds())
                .chain(objects.iter().map(|o| o.bounds()))
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

    fn build_tree() {}

    fn combine_clusters() {}
}

impl<T> Boundable for Bvh<T> where T: Boundable {
    fn bounds(&self) -> Aabb {
        self.aabb
    }
}

impl<T> Intersectable for Bvh<T> where T: Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounds().intersects(ray) {
            return None;
        }

        let obj_intersection = self
            .objects
            .iter()
            .filter_map(|o| o.intersect(ray))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        if let Some(obj) = obj_intersection {
            let mut new_ray = *ray;
            new_ray.t_end = obj.t;

            let child_intersection = self
                .children
                .iter()
                .filter_map(|c| c.intersect(ray))
                .min_by(|a, b| floats::fast_cmp(a.t, b.t));

            if let Some(child) = child_intersection {
                if child.t < obj.t {
                    return child_intersection;
                }
            }

            obj_intersection
        } else {
            self.children
                .iter()
                .filter_map(|c| c.intersect(ray))
                .min_by(|a, b| floats::fast_cmp(a.t, b.t))
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.objects.iter().any(|o| o.intersects(ray))
            || self.children.iter().any(|c| c.intersects(ray))
    }
}