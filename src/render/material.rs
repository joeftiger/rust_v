use crate::Spectrum;
use crate::render::bxdf::bsdf::BSDF;
use color::Color;
use ultraviolet::Vec3;

pub struct Material {
    pub emission: Spectrum,
    pub bsdf: BSDF,
}

impl Material {
    pub fn new(emission: Spectrum, bsdf: BSDF) -> Self {
        Self { emission, bsdf }
    }

    pub fn emissive(&self) -> bool {
        !self.emission.is_black()
    }

    pub fn radiance(&self, outgoing: &Vec3, normal: &Vec3) -> Spectrum {
        if outgoing.dot(*normal) > 0.0 {
            self.emission
        } else {
            Spectrum::black()
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::from(BSDF::empty())
    }
}

impl From<BSDF> for Material {
    fn from(bsdf: BSDF) -> Self {
        Self::new(Spectrum::black(), bsdf)
    }
}
