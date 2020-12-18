use crate::bxdf::bsdf::BSDF;
use crate::Spectrum;
use color::Color;
use ultraviolet::Vec3;

#[derive(Debug)]
pub struct Material {
    pub emission: Option<Spectrum>,
    pub bsdf: BSDF,
}

impl Material {
    pub fn new(emission: Option<Spectrum>, bsdf: BSDF) -> Self {
        Self { emission, bsdf }
    }

    pub fn emissive(&self) -> bool {
        self.emission.is_some()
    }

    pub fn radiance(&self, outgoing: &Vec3, normal: &Vec3) -> Spectrum {
        if let Some(emission) = self.emission {
            if outgoing.dot(*normal) > 0.0 {
                emission
            } else {
                Spectrum::black()
            }
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
        Self::new(None, bsdf)
    }
}
