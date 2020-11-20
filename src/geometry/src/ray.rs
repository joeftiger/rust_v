use ultraviolet::{f32x4, Vec3, Vec3x4};

macro_rules! rays {
    ($($name:ident => $vec:ident, $float:ident), +) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name {
                pub origin: $vec,
                pub direction: $vec,
                pub t_start: $float,
                pub t_end: $float
            }

            impl $name {
                pub fn new(origin: $vec, direction: $vec) -> Self {
                    let t_start = 0.0.into();
                    let t_end = f32::INFINITY.into();
                    Self { origin, direction, t_start, t_end }
                }

                pub fn with(origin: $vec, direction: $vec, t_start: $float, t_end: $float) -> Self {
                    Self { origin, direction, t_start, t_end }
                }

                pub fn in_range(from: &$vec, to: &$vec) -> Self {
                    let origin = *from;
                    let mut direction = *to - *from;
                    let t_start = 0.0.into();
                    let t_end = direction.mag();
                    direction.normalize();

                    Self { origin, direction, t_start, t_end }
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
        self.origin == other.origin
            && self.direction == other.direction
            && self.t_start == other.t_start
            && self.t_end == other.t_end
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
            && self.t_start == other.t_start
            && self.t_end == other.t_end
    }
}

impl Ray {
    #[inline(always)]
    pub fn is_in_range(&self, t: f32) -> bool {
        t >= self.t_start && t <= self.t_end
    }

    #[inline]
    pub fn is_in_range_op(&self, t: f32) -> Option<f32> {
        if self.is_in_range(t) {
            Some(t)
        } else {
            None
        }
    }
}
