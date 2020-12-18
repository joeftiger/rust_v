use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::{Container, Intersection, Boundable, Intersectable};
use ultraviolet::Vec3;
use util::MinMaxExt;
use util::math;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Capsule {
    pub a: Vec3,
    pub b: Vec3,
    pub radius: f32,
}

impl Capsule {
    pub fn new(a: Vec3, b: Vec3, radius: f32,) -> Self {
        Self { a, b, radius }
    }
}

impl Boundable for Capsule {
    fn bounds(&self) -> Aabb {
        let radius = Vec3::one() * self.radius;
        let min = self.a.min_by_component(self.b) - radius;
        let max = self.a.max_by_component(self.b) + radius;

        Aabb::new(min, max)
    }
}

impl Container for Capsule {
    fn contains(&self, obj: &Vec3) -> bool {
        self.bounds().contains(obj)
    }
}

impl Intersectable for Capsule {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let ab = self.b - self.a;
        let ao = ray.origin - self.a;

        let ab_dot_d = ab.dot(ray.direction);
        let ab_dot_ao = ab.dot(ao);
        let ab_dot_ab = ab.dot(ab);

        let m = ab_dot_d / ab_dot_ab;
        let n = ab_dot_ao / ab_dot_ab;

        let q = ray.direction - (ab * m);
        let r = ao - (ab * n);

        let a = q.dot(q);
        let b = 2.0 * q.dot(r);
        let c = r.dot(r) - self.radius * self.radius;

        let (t0, t1) = math::solve_quadratic(a, b, c)?;
        let t_min = f32::mmin_op2(ray.is_in_range_op(t0), ray.is_in_range_op(t1))?;

        // check against line segment of [self.a, self.b]
        let axis_shift = t_min * m + n;
        if axis_shift < 0.0 {
            // on sphere A
            Sphere::new(self.a, self.radius).intersect(ray)
        } else if axis_shift > 1.0 {
            // on sphere B
            Sphere::new(self.b, self.radius).intersect(ray)
        } else {
            // on cylinder
            let point = ray.at(t_min);
            let on_axis = self.a + ab * axis_shift;
            let normal = point - on_axis;

            Some(Intersection::new(*ray, t_min, point, normal))
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let ab = self.b - self.a;
        let ao = ray.origin - self.a;

        let ab_dot_d = ab.dot(ray.direction);
        let ab_dot_ao = ab.dot(ao);
        let ab_dot_ab = ab.dot(ab);

        let m = ab_dot_d / ab_dot_ab;
        let n = ab_dot_ao / ab_dot_ab;

        let q = ray.direction - (ab * m);
        let r = ao - (ab * n);

        let a = q.dot(q);
        let b = 2.0 * q.dot(r);
        let c = r.dot(r) - self.radius * self.radius;

        if let Some((t0, t1)) = math::solve_quadratic(a, b, c) {
            if let Some(t_min) = f32::mmin_op2(ray.is_in_range_op(t0), ray.is_in_range_op(t1)) {

                // check against line segment of [self.a, self.b]
                let axis_shift = t_min * m + n;
                if axis_shift < 0.0 {
                    // on sphere A
                    return Sphere::new(self.a, self.radius).intersects(ray);
                } else if axis_shift > 1.0 {
                    // on sphere B
                    return Sphere::new(self.b, self.radius).intersects(ray);
                } else {
                    return true;
                }
            }
        }

        false
    }
}