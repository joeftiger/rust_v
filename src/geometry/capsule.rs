use crate::geometry::aabb::Aabb;
use crate::geometry::cylinder::Cylinder;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::{Container, Geometry, GeometryInfo};
use ultraviolet::Vec3;
use crate::util::MinMaxExt;

pub struct Capsule {
    top: Sphere,
    bot: Sphere,
    cylinder: Cylinder,
}

impl Capsule {
    pub fn new(from: Vec3, to: Vec3, radius: f32) -> Self {
        let center = (from + to) / 2.0;
        let axis = to - from;

        let cylinder = Cylinder::new(center, axis.normalized(), radius, axis.mag());
        let top = Sphere::new(from, radius);
        let bot = Sphere::new(to, radius);

        Self { top, bot, cylinder }
    }
}

impl Container for Capsule {
    fn contains(&self, obj: Vec3) -> bool {
        self.top.contains(obj) || self.bot.contains(obj)
    }
}

impl Geometry for Capsule {
    fn bounding_box(&self) -> Aabb {
        self.bot.bounding_box().outer_join(&self.top.bounding_box())
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let bot = self.bot.intersect(ray);
        let top = self.top.intersect(ray);
        let cylinder = self.cylinder.intersect(ray);

        GeometryInfo::mmin_op2(bot, GeometryInfo::mmin_op2(top, cylinder))
    }
}
