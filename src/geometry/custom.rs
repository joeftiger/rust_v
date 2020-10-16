use std::borrow::Borrow;

use crate::geometry::{Boxable, Intersectable, Intersection, Ray};
use crate::geometry::aabb::Aabb;

pub struct CustomShape<T> {
    base: T,
    additions: Vec<T>,
    cutouts: Vec<T>,
}

impl<T> CustomShape<T> {
    fn new(base: T, additions: Vec<T>, cutouts: Vec<T>) -> Self {
        Self { base, additions, cutouts }
    }
}

impl<T> From<T> for CustomShape<T> {
    fn from(base: T) -> Self {
        Self { base, additions: vec![], cutouts: vec![] }
    }
}

impl<T: Boxable> Boxable for CustomShape<T> {
    fn bounding_box(&self) -> Option<Aabb> {
        self.base.bounding_box()
    }
}

impl<T: Intersectable<Ray>> Intersectable<Ray> for CustomShape<T> {
    // noinspection DuplicatedCode
    fn intersects(&self, ray: Ray) -> Option<Intersection> {
        if let Some(mut intersection) = self.base.intersects(ray) {
            // get closest addition intersection
            for a in &self.additions {
                if let Some(i) = a.intersects(ray) {
                    if i.t[0] > 0.0 && i.t[0] < intersection.t[0] {
                        intersection = i;
                    }
                }
            }

            // get closest cutout intersection
            for c in &self.cutouts {
                if let Some(i) = c.intersects(ray) {
                    for t in i.clone().t {
                        if t > 0.0 && t > intersection.t[0] {
                            intersection = i;
                            break;
                        }
                    }
                }
            }

            return Some(intersection);
        }

        None
    }
}
