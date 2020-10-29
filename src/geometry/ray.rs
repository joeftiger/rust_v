use ultraviolet::{f32x4, Vec3, Vec3x4};

macro_rules! rays {
    ($($name:ident => $vec:ident, $float:ident), +) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name {
                pub origin: $vec,
                pub direction: $vec,
                pub t: $float,
            }

            impl $name {
                pub fn new(origin: $vec, direction: $vec, t: $float) -> Self {
                    Self { origin, direction, t }
                }

                pub fn at(&self, t: $float) -> $vec {
                    self.direction.mul_add($vec::new(t, t, t), self.origin)
                }
            }
        )+
    }
}

rays!(
    Ray => Vec3, f32,
    Ray4 => Vec3x4, f32x4
);

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction && self.t == other.t
    }
}

impl PartialEq for Ray4 {
    fn eq(&self, other: &Self) -> bool {
        self.origin.x == other.origin.x
            && self.origin.y == other.origin.y
            && self.origin.z == other.origin.z
            && self.direction.x == other.direction.x
            && self.direction.y == other.direction.y
            && self.direction.z == other.direction.z
            && self.t == other.t
    }
}
