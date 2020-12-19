use ultraviolet::Vec3;
use util::floats;

use crate::ray::Ray;
use crate::{ComparableExt, Container, DistanceExt, Intersection, Boundable, Intersectable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        debug_assert!(min.x <= max.x);
        debug_assert!(min.y <= max.y);
        debug_assert!(min.z <= max.z);

        Self { min, max }
    }

    pub fn infinite() -> Self {
        let min = Vec3::one() * f32::NEG_INFINITY;
        let max = Vec3::one() * f32::INFINITY;
        Self { min, max }
    }

    /// Be careful with this one!
    /// It can be used to outer join many aabbs with each other.
    pub fn inverted_infinite() -> Self {
        let min = Vec3::one() * f32::INFINITY;
        let max = Vec3::one() * f32::NEG_INFINITY;
        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3 {
        (self.max + self.min) / 2.0
    }

    #[inline]
    #[must_use]
    pub fn max_radius(&self) -> f32 {
        self.size().mag() / 2.0
    }

    #[inline]
    #[must_use]
    pub fn min_radius(&self) -> f32 {
        self.size().component_min() / 2.0
    }

    /// Creates the inner join / intersection of both aabbs.
    pub fn inner_join(&self, other: &Self) -> Self {
        let min = self.min.max_by_component(other.min);
        let max = self.max.min_by_component(other.max);

        Self::new(min, max)
    }

    /// Creates the outer join / overlap of both aabbs.
    pub fn outer_join(&self, other: &Self) -> Self {
        let min = self.min.min_by_component(other.min);
        let max = self.max.max_by_component(other.max);

        Self::new(min, max)
    }

    /// Do the two aabbs overlap
    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.lt(&other.max) && self.max.gt(&other.min)
    }

    /// Returns the corners of this aabb from bottom to top in the order of:
    /// * left back
    /// * right back
    /// * right front
    /// * left front
    #[allow(clippy::many_single_char_names)]
    pub fn corners(&self) -> [Vec3; 8] {
        let min = self.min;
        let max = self.max;

        let a = min;
        let b = Vec3::new(max.x, min.y, min.z);
        let c = Vec3::new(max.x, min.y, max.z);
        let d = Vec3::new(min.x, min.y, max.z);
        let e = Vec3::new(min.x, max.y, min.z);
        let f = Vec3::new(max.x, max.y, min.z);
        let g = max;
        let h = Vec3::new(min.x, max.y, max.z);

        [a, b, c, d, e, f, g, h]
    }
}

impl Boundable for Aabb {
    fn bounds(&self) -> Aabb {
        *self
    }
}

impl Container for Aabb {
    fn contains(&self, obj: &Vec3) -> bool {
        *obj == obj.clamped(self.min, self.max)
    }
}

impl Intersectable for Aabb {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        if !(t_max >= ray.t_start && t_max >= t_min && t_min <= ray.t_end) {
            return None;
        }

        let hit = ray.at(t_min);
        let point = hit - self.center();
        let extent = self.size() / 2.0;
        let bias = 1.0 + floats::BIG_EPSILON;

        let mut normal = point * bias / extent;
        normal.x = (normal.x as i32) as f32;
        normal.y = (normal.y as i32) as f32;
        normal.z = (normal.z as i32) as f32;

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        // approximating epsilon is too small (unlikely) or the given hit was illegal
        Some(Intersection::new(*ray, t_min, hit, normal))
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        t_max >= ray.t_start && t_max >= t_min && t_min <= ray.t_end
    }
}

// impl Geometry for Aabb {
//     fn surface_area(&self) -> f32 {
//         let size = self.size();
//         2.0 * (size.x * size.y + size.x * size.z + size.y * size.z)
//     }
//
//     #[rustfmt::skip]
//     fn sample_surface(&self, sample: &Vec3) -> Vec3 {
//         let x;
//         let y;
//         let z;
//         if sample.x < sample.y && sample.x < sample.z {
//             x = if sample.y < sample.z { self.min.x } else { self.max.x };
//             y = self.min.y.lerp(self.max.y, sample.y);
//             z = self.min.z.lerp(self.max.z, sample.z);
//         } else if sample.y < sample.z && sample.y < sample.x {
//             x = self.min.x.lerp(self.max.x, sample.x);
//             y = if sample.z < sample.x { self.min.y } else { self.max.y };
//             z = self.min.z.lerp(self.max.z, sample.x);
//         } else {
//             x = self.min.x.lerp(self.max.x, sample.x);
//             y = self.min.y.lerp(self.max.y, sample.y);
//             z = if sample.x < sample.y { self.min.z } else { self.max.z };
//         }
//
//         debug_assert!(!x.is_nan());
//         debug_assert!(!y.is_nan());
//         debug_assert!(!z.is_nan());
//
//         Vec3::new(x, y, z)
//     }
// }

impl Default for Aabb {
    fn default() -> Self {
        let min = -Vec3::one();
        let max = Vec3::one();

        Self::new(min, max)
    }
}

impl DistanceExt for Aabb {
    fn distance(&self, other: &Self) -> f32 {
        self.center().distance(&other.center())
    }
}
