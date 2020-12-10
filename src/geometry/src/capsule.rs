use crate::aabb::Aabb;
use crate::cylinder::Cylinder;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::{Container, Geometry, GeometryInfo};
use ultraviolet::Vec3;
use util::MinMaxExt;

#[derive(Debug, PartialEq)]
pub struct Capsule {
    top: Sphere,
    bot: Sphere,
    cylinder: Cylinder,
}

impl Capsule {
    pub fn new(from: Vec3, to: Vec3, radius: f32) -> Self {
        let cylinder = Cylinder::new(from, to, radius);
        let top = Sphere::new(from, radius);
        let bot = Sphere::new(to, radius);

        Self { top, bot, cylinder }
    }
}

impl Container for Capsule {
    fn contains(&self, obj: Vec3) -> bool {
        self.top.contains(obj) || self.bot.contains(obj) || self.cylinder.contains(obj)
    }
}

impl Geometry for Capsule {
    fn bounding_box(&self) -> Aabb {
        self.bot.bounding_box().outer_join(&self.top.bounding_box())
    }

    fn sample_surface(&self, sample: &Vec3) -> Vec3 {
        let radius = self.bot.radius;
        let height = self.cylinder.height();
        let total = radius + radius + height;

        let max = sample.component_max();
        if max < radius / total {
            self.bot.sample_surface(sample)
        } else if max < (radius + radius) / total {
            self.top.sample_surface(sample)
        } else {
            self.cylinder.sample_surface(sample)
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        // FIXME: Intersections from the inside are not handled correctly!
        let bot = self.bot.intersect(ray);
        let top = self.top.intersect(ray);
        let cylinder = self.cylinder.intersect(ray);

        GeometryInfo::mmin_op2(bot, GeometryInfo::mmin_op2(top, cylinder))
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.cylinder.intersects(ray) || self.bot.intersects(ray) || self.top.intersects(ray)
    }
}
