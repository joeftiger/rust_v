use crate::*;
use image::Rgb;
use util::floats;
use ultraviolet::Vec3;

colors!(
    Srgb => f32, f32, 3
);

impl Srgb {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::from(self.data)
    }
}

impl Color for Srgb {
    fn is_black(&self) -> bool {
        floats::approx_zero_ar(&self.data)
    }

    fn clamp(&self, min: f32, max: f32) -> Self {
        self.clamp(min, max)
    }

    fn has_nans(&self) -> bool {
        self.data.iter().all(|value| !value.is_nan())
    }

    fn sqrt(&self) -> Self {
        Self::sqrt(self)
    }

    fn to_rgb(&self) -> Srgb {
        *self
    }

    fn to_xyz(&self) -> Xyz {
        Xyz::from(srgb_to_xyz_mat() * srgbs_to_linear(self.to_vec3()))
    }

    fn black() -> Self {
        Self::new([0.0, 0.0, 0.0])
    }

    fn white() -> Self {
        Self::new([1.0, 1.0, 1.0])
    }

    fn red() -> Self {
        Self::new([1.0, 0.0, 0.0])
    }

    fn green() -> Self {
        Self::new([0.0, 1.0, 0.0])
    }

    fn blue() -> Self {
        Self::new([0.0, 0.0, 1.0])
    }
}

impl Into<Rgb<u8>> for Srgb {
    fn into(self) -> Rgb<u8> {
        let mut data = [0; 3];
        data.iter_mut()
            .zip(self.data.iter())
            .for_each(|(d0, d1)| *d0 = (d1.max(0.0).min(1.0) * 2u32.pow(8) as f32) as u8);

        Rgb::from(data)
    }
}

impl Into<Rgb<u16>> for Srgb {
    fn into(self) -> Rgb<u16> {
        let mut data = [0; 3];
        data.iter_mut()
            .zip(self.data.iter())
            .for_each(|(d0, d1)| *d0 = (d1 * 2u32.pow(16) as f32) as u16);

        Rgb::from(data)
    }
}

impl From<Vec3> for Srgb {
    fn from(vec: Vec3) -> Self {
        Self::new([vec.x, vec.y, vec.z])
    }
}
