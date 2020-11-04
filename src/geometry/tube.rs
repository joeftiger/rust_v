use crate::geometry::aabb::Aabb;
use crate::geometry::cylinder::Cylinder;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::{Container, Geometry, GeometryInfo};
use crate::util::MinMaxExt;
use ultraviolet::Vec3;

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
            let c = Cylinder::new2(points[i], points[i + 1], radius);
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
}
