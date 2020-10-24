use crate::color::Srgb;
use ultraviolet::{Vec3, Vec4};

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

                pub fn set_radiance(&mut self, radiance: $rad) {
                    self.radiance = radiance;
                }

                pub fn set_polarization(&mut self, polarization: $stokes) {
                    self.polarization = polarization;
                }

                pub fn at(&self, t: $float) -> $vec {
                    self.direction.mul_add($vec::new(t, t, t), self.origin)
                }
            }
        )+
    }
}

rays!(
    Ray => Vec3, Srgb, Vec4, f32
);

impl Ray {
    pub fn new_simple(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            radiance: Srgb::from(Vec3::zero()),
            polarization: Vec4::zero(),
        }
    }
}
