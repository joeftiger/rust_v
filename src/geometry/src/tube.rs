use crate::aabb::Aabb;
use crate::cylinder::Cylinder;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::{Container, IntersectionInfo, Boundable, Intersectable};
use ultraviolet::Vec3;
use util::MinMaxExt;

#[derive(Debug, PartialEq)]
pub struct Tube {
    pub cylinders: Vec<Cylinder>,
    pub spheres: Vec<Sphere>,
    pub radius: f32,
    pub aabb: Aabb,
}

impl Tube {
    pub fn new(points: &[Vec3], radius: f32) -> Self {
        let mut spheres = Vec::with_capacity(points.len());
        points
            .iter()
            .for_each(|p| spheres.push(Sphere::new(*p, radius)));
        if points.len() <= 1 {
            return Self {
                cylinders: Vec::with_capacity(0),
                spheres,
                radius,
                aabb: Aabb::inverted_infinite(),
            };
        }

        let capacity = points.len() - 1;

        let mut cylinders = Vec::with_capacity(capacity);
        (0..capacity).for_each(|i| {
            let c = Cylinder::new(points[i], points[i + 1], radius);
            cylinders.push(c);
        });

        let mut aabb = Aabb::inverted_infinite();
        for s in &spheres {
            aabb = aabb.outer_join(&s.bounds());
        }

        Self {
            cylinders,
            spheres,
            radius,
            aabb,
        }
    }
}

impl Boundable for Tube {
    fn bounds(&self) -> Aabb {
        self.aabb.clone()
    }
}

impl Container for Tube {
    fn contains(&self, obj: &Vec3) -> bool {
        self.spheres.iter().any(|s| s.contains(obj))
            || self.cylinders.iter().any(|c| c.contains(obj))
    }
}

impl Intersectable for Tube {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let mut intersection = None;

        self.spheres.iter().for_each(|sphere| {
            intersection = IntersectionInfo::mmin_op2(intersection, sphere.intersect(ray))
        });

        self.cylinders.iter().for_each(|cylinder| {
            intersection = IntersectionInfo::mmin_op2(intersection, cylinder.intersect(ray))
        });

        intersection
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.cylinders.iter().any(|c| c.intersects(ray))
            || self.spheres.iter().any(|s| s.intersects(ray))
    }
}