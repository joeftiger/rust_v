// #![allow(dead_code)]
//
// use std::sync::Arc;
//
// use util::{floats, MinMaxExt};
//
// use crate::{Geometry, GeometryInfo, DistanceExt};
// use crate::aabb::Aabb;
// use crate::ray::Ray;
//
// pub fn build_tree(geometries: Vec<Arc<dyn Geometry>>) -> BvhNode {
//     unimplemented!();
//
//     let mut boxes: Vec<(usize, Aabb)> = geometries.iter().map(|g| g.bounding_box()).enumerate().collect();
//     let mut nodes: Vec<Arc<BvhNode>> = Vec::default();
//
//     while !boxes.is_empty() || nodes.len() > 1 {
//         // Get the closest geometries/nodes and make parent node
//         let mut closest: Option<(Arc<dyn Geometry>, Arc<dyn Geometry>)> = None;
//         let mut distance = f32::INFINITY;
//
//         for i in 0..boxes.len() {
//             for j in (i + 1)..boxes.len() {
//                 let d = boxes[i].1.distance(&boxes[j].1);
//                 if d < distance {
//                     closest = Some((geometries[i].clone(), geometries[j].clone()));
//                 }
//             }
//
//             for j in 0..nodes.len() {
//                 let d = boxes[i].1.distance(&nodes[i].bounding_box());
//                 if d < distance {
//                     closest = Some((geometries[i].clone(), nodes[i].clone()));
//                 }
//             }
//         }
//
//         if let Some(closest) = closest {
//             let aabb = closest.0.bounding_box().outer_join(&closest.1.bounding_box());
//             let geometries = vec![closest.0, closest.1];
//
//             let node = BvhNode::new(aabb, geometries);
//             nodes.push(Arc::new(node));
//         }
//     }
//
//
//     let aabb = geometries
//         .iter()
//         .fold(Aabb::inverted_infinite(), |aabb, next| aabb.outer_join(&next.bounding_box()));
//     if geometries.len() <= 1 {
//         return BvhNode::new(aabb, Vec::new(), geometries);
//     }
//
//
//
//     node
// }
//
//
// #[derive(Default)]
// pub struct BvhNode {
//     aabb: Aabb,
//     geometries: Vec<Arc<dyn Geometry>>,
// }
//
// impl BvhNode {
//     pub fn new_empty() -> Self {
//         let aabb = Aabb::inverted_infinite();
//         let geometries = vec![];
//
//         Self::new(aabb, geometries)
//     }
//
//     pub fn new(aabb: Aabb,
//                geometries: Vec<Arc<dyn Geometry>>) -> Self {
//         Self { aabb, geometries }
//     }
//
//     pub fn shrink_to_fit(&mut self) {
//         self.geometries.shrink_to_fit()
//     }
//
//     pub fn clear(&mut self) {
//         self.geometries.clear();
//     }
// }
//
// impl Geometry for BvhNode {
//     fn bounding_box(&self) -> Aabb {
//         self.aabb.clone()
//     }
//
//     fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
//         if !self.aabb.intersects(ray) {
//             return None;
//         }
//
//         self.geometries
//             .iter()
//             .filter_map(|g| g.intersect(ray))
//             .min_by(|a, b| floats::fast_cmp(a.t, b.t))
//     }
//
//     fn intersects(&self, ray: &Ray) -> bool {
//         if self.aabb.intersects(ray) {
//             self.geometries.iter().any(|g| g.intersects(ray))
//         } else {
//             false
//         }
//     }
// }
