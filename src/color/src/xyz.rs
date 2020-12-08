use crate::*;
use image::Rgb;
use util::floats;
use ultraviolet::Vec3;

colors!(
    Xyz => f32, f32, 3
);

impl Xyz {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::from(self.data)
    }
}

impl Color for Xyz {
    fn is_black(&self) -> bool {
        self.data.iter().all(|value| floats::approx_zero(*value))
    }

    fn clamp(&self, min: f32, max: f32) -> Self {
        let mut data = self.data;
        floats::fast_clamp_ar(&mut data, min, max);

        Self::new(data)
    }

    fn has_nans(&self) -> bool {
        self.data.iter().all(|value| !value.is_nan())
    }

    fn sqrt(&self) -> Self {
        Self::sqrt(self)
    }

    fn to_rgb(&self) -> Srgb {
        Srgb::from(linears_to_srgb(xyz_to_srgb_mat() * self.to_vec3()))
    }

    fn to_xyz(&self) -> Xyz {
        *self
    }

    fn black() -> Self {
        Srgb::black().to_xyz()
    }

    fn white() -> Self {
        Srgb::white().to_xyz()
    }

    fn red() -> Self {
        Srgb::red().to_xyz()
    }

    fn green() -> Self {
        Srgb::green().to_xyz()
    }

    fn blue() -> Self {
        Srgb::blue().to_xyz()
    }
}

impl Into<Rgb<u8>> for Xyz {
    fn into(self) -> Rgb<u8> {
        self.to_rgb().into()
    }
}

impl Into<Rgb<u16>> for Xyz {
    fn into(self) -> Rgb<u16> {
        self.to_rgb().into()
    }
}

impl From<Vec3> for Xyz {
    fn from(vec: Vec3) -> Self {
        Self::new([vec.x, vec.y, vec.z])
    }
}
