use crate::color::Srgb;
use ultraviolet::{f32x4, Vec3, Vec3x4, Vec4, Vec4x4};

macro_rules! rays {
    ($($name:ident => $vec:ident, $rad:ident, $stokes:ident, $float:ident), +) => {
        $(
            #[derive(Clone, Copy)]
            pub struct $name {
                pub origin: $vec,
                pub direction: $vec,
                pub radiance: $rad,
                pub polarization: $stokes,
            }

            impl $name {
                pub fn new(origin: $vec, direction: $vec, radiance: $rad, polarization: $stokes) -> Self {
                    Self { origin, direction: direction.normalized(), radiance, polarization }
                }

                pub fn new_simple(origin: $vec, direction: $vec) -> Self {
                    Self::new(origin, direction, $rad::default(), $stokes::zero())
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

            impl From<&Self> for $name {
                fn from(ray: &Self) -> Self {
                    Self::new(ray.origin, ray.direction, ray.radiance, ray.polarization)
                }
            }
        )+
    }
}

rays!(
    Ray => Vec3, Srgb, Vec4, f32,
    Ray4 => Vec3x4, Srgb, Vec4x4, f32x4
);
