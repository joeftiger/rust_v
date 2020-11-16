use crate::render::bxdf::{BxDF, BxDFType};
use crate::Spectrum;
use std::f32::consts::FRAC_1_PI;
use ultraviolet::Vec3;

#[derive(Debug)]
pub struct LambertianReflection {
    r: Spectrum,
}

impl LambertianReflection {
    pub fn new(r: Spectrum) -> Self {
        Self { r }
    }
}

impl BxDF for LambertianReflection {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::DIFFUSE
    }

    fn evaluate(&self, _incident: &Vec3, _outgoing: &Vec3) -> Spectrum {
        self.r * FRAC_1_PI
    }
}

#[derive(Debug)]
pub struct LambertianTransmission {
    t: Spectrum,
}

impl LambertianTransmission {
    pub fn new(t: Spectrum) -> Self {
        Self { t }
    }
}

impl BxDF for LambertianTransmission {
    fn get_type(&self) -> BxDFType {
        BxDFType::DIFFUSE | BxDFType::TRANSMISSION
    }

    fn evaluate(&self, _incident: &Vec3, _outgoing: &Vec3) -> Spectrum {
        self.t * FRAC_1_PI
    }
}
