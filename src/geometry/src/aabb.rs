use util::floats;
use crate::ray::Ray;
use crate::{ComparableExt, Container, Geometry, GeometryInfo};
use ultraviolet::Vec3;

#[derive(Clone, Debug, PartialEq)]
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
}

impl Container for Aabb {
    fn contains(&self, obj: Vec3) -> bool {
        let clamped = obj.clamped(self.min, self.max);

        clamped == obj
    }
}

impl Geometry for Aabb {
    fn bounding_box(&self) -> Aabb {
        self.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        if t_max < 0.0 || t_max < t_min || t_min < ray.t_start || ray.t_end < t_min {
            return None;
        }

        let point = ray.at(t_min);
        let min = point - self.min;
        let max = point - self.max;

        let mut closest = None;

        // left
        closest = compare_closest(min.x, -Vec3::unit_x(), closest);
        // right
        closest = compare_closest(max.x, Vec3::unit_x(), closest);
        // down
        closest = compare_closest(min.y, -Vec3::unit_y(), closest);
        // up
        closest = compare_closest(max.y, Vec3::unit_y(), closest);
        // back
        closest = compare_closest(min.z, -Vec3::unit_z(), closest);
        // forward
        closest = compare_closest(max.z, Vec3::unit_z(), closest);

        let closest = closest?;
        let mut normal = closest.1;

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        // approximating epsilon is too small (unlikely) or the given hit was illegal
        Some(GeometryInfo::new(*ray, t_min, point, normal))
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let t1 = (self.min - ray.origin) / ray.direction;
        let t2 = (self.max - ray.origin) / ray.direction;

        let t_min_vec = t1.min_by_component(t2);
        let t_max_vec = t1.max_by_component(t2);

        let t_min = f32::max(t_min_vec.z, f32::max(t_min_vec.y, t_min_vec.x));
        let t_max = f32::min(t_max_vec.z, f32::min(t_max_vec.y, t_max_vec.x));

        t_max >= ray.t_start && t_min <= ray.t_end && t_max >= t_min
    }
}

fn compare_closest(d: f32, v: Vec3, closest: Option<(f32, Vec3)>) -> Option<(f32, Vec3)> {
    if floats::approx_zero(d) {
        if let Some((dist, _)) = closest {
            if d < dist {
                return Some((d, v));
            }
        } else {
            return Some((d, v));
        }
    }
    closest
}

impl Default for Aabb {
    fn default() -> Self {
        let min = -Vec3::one();
        let max = Vec3::one();

        Self::new(min, max)
    }
}
