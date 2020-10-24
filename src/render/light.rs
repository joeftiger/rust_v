use ultraviolet::Vec3;

use crate::color::Srgb;
use crate::geometry::aabb::Aabb;
use crate::geometry::cube::Cube;
use crate::geometry::intersection::Intersection;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::Geometry;

macro_rules! lights {
    ($($name:ident => $vec:ident, $ray:ident, $shape:ident, $rad:ident), +) => {
        $(
            pub struct $name {
                pub position: $vec,
                pub shape: $shape,
                pub radiance: $rad,
            }

            impl $name {
                pub fn ray_to(&self, position: $vec) -> $ray {
                    let mut ray = $ray::new_simple(self.position, position);
                    ray.set_radiance(self.radiance.clone());
                    ray
                }

                pub fn distance(&self, position: $vec) -> f32 {
                    (position - self.position).mag()
                }
            }

            impl Geometry<$ray, Intersection> for $name {
                fn bounding_box(&self) -> Aabb {
                    self.shape.bounding_box()
                }

                fn intersect(&self, ray: &Ray) -> Option<Intersection> {
                    self.shape.intersect(ray)
                }
            }
        )+
    };
}

lights!(
    PointLight => Vec3, Ray, Point, Srgb,
    SphericalLight => Vec3, Ray, Sphere, Srgb,
    CubicLight => Vec3, Ray, Cube, Srgb
);
