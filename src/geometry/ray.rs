use ultraviolet::{f32x4, Vec3, Vec3x4};

macro_rules! rays {
    ($($name:ident => $vec:ident, $float:ident), +) => {
        $(
            #[derive(Clone, Copy)]
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
