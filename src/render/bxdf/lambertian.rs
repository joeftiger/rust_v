use crate::render::bxdf::{BxDF, BxDFSample, BxDFType};
use crate::Spectrum;
use std::f32::consts::PI;
use ultraviolet::{Vec2, Vec3};

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

    fn evaluate(&self, normal: Vec3, _: Vec3, outgoing: Vec3) -> Spectrum {
        self.r * normal.dot(outgoing).max(0.0) / PI
    }

    fn sample(&self, _normal: Vec3, _outgoing: Vec3, _sample: Vec2) -> BxDFSample {
        unimplemented!()
    }
}

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

    fn evaluate(&self, normal: Vec3, _: Vec3, outgoing: Vec3) -> Spectrum {
        self.t * normal.dot(outgoing).max(0.0) / PI
    }
}
