use crate::aabb::Aabb;
use crate::cylinder::Cylinder;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::{Container, Geometry, GeometryInfo};
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
            aabb = aabb.outer_join(&s.bounding_box());
        }

        Self {
            cylinders,
            spheres,
            radius,
            aabb,
        }
    }
}

impl Container for Tube {
    fn contains(&self, obj: Vec3) -> bool {
        self.spheres.iter().any(|s| s.contains(obj))
            || self.cylinders.iter().any(|c| c.contains(obj))
    }
}

impl Geometry for Tube {
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    fn sample_surface(&self, _sample: &Vec3) -> Vec3 {
        let heights: Vec<f32> = self.cylinders.iter().map(|c| c.height()).collect();
        let total_radiuses = self.radius * self.spheres.len() as f32;
        let total_height: f32 = heights.iter().sum();
        let _total = (total_radiuses + total_height) as usize;

        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let mut intersection = None;

        self.spheres.iter().for_each(|sphere| {
            intersection = GeometryInfo::mmin_op2(intersection, sphere.intersect(ray))
        });

        self.cylinders.iter().for_each(|cylinder| {
            intersection = GeometryInfo::mmin_op2(intersection, cylinder.intersect(ray))
        });

        intersection
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.cylinders.iter().any(|c| c.intersects(ray))
            || self.spheres.iter().any(|s| s.intersects(ray))
    }
}
