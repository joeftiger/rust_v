use ultraviolet::{f32x4, Vec3, Vec3x4};
use std::cmp::Ordering;

macro_rules! intersections {
    ($($name:ident => $float:ident, $vec:ident), +) => {
        $(
        pub struct $name {
            pub t: $float,
            pub point: $vec,
            pub normal: $vec,
        }

        impl $name {
            pub fn new(t: $float, point: $vec, normal: $vec) -> Self {
                Self { t, point, normal }
            }
        })+
    }
}

intersections!(
    Intersection => f32, Vec3,
    Intersection4 => f32x4, Vec3x4
);

impl Intersection {
    pub fn cmp(&self, other: &Self) -> Ordering {
        debug_assert!(!self.t.is_nan());
        debug_assert!(!other.t.is_nan());
        
        if self.t < other.t {
            Ordering::Greater
        } else if self.t > other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
