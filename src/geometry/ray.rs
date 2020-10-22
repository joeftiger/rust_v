use ultraviolet::{f32x4, Vec3, Vec3x4, Vec4, Vec4x4};

macro_rules! rays {
    ($($name:ident => $vec:ident, $rad:ident, $stokes:ident, $float:ident), +) => {
        $(
        pub struct $name {
            pub origin: $vec,
            pub direction: $vec,
            pub radiance: $rad,
            pub polarization: $stokes,
        }

        impl $name {
            pub fn new(origin: $vec, direction: $vec, radiance: $rad, polarization: $stokes) -> Self {
                Self { origin, direction, radiance, polarization }
            }

            pub fn at(&self, t: $float) -> $vec {
                self.direction.mul_add($vec::new(t, t, t), self.origin)
            }
        })+
    }
}

rays!(
    Ray => Vec3, Vec3, Vec4, f32,
    Ray4 => Vec3x4, Vec3x4, Vec4x4, f32x4
);

impl Ray {
    pub fn new_simple(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            radiance: Vec3::zero(),
            polarization: Vec4::zero(),
        }
    }
}

impl Ray4 {
    pub fn new_simple(origin: Vec3x4, direction: Vec3x4) -> Self {
        Self {
            origin,
            direction,
            radiance: Vec3x4::zero(),
            polarization: Vec4x4::zero(),
        }
    }
}
