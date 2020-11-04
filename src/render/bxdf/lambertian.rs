use crate::render::bxdf::{BxDF, BxDFType};
use crate::Spectrum;
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

    fn apply(&self, normal: Vec3, _: Vec3, from: Vec3) -> Spectrum {
        self.r * normal.dot(from).max(0.0) / std::f32::consts::PI
    }

    #[allow(unused_variables)]
    fn apply_sample(
        &self,
        normal: Vec3,
        view: Vec3,
        from: Vec3,
        sample: Vec2,
        pdf: f32,
        sampled_type: BxDFType,
    ) -> Spectrum {
        unimplemented!()
    }

    fn rho(&self, _: Vec3, _: u32, _: Vec2) -> Spectrum {
        self.r
    }

    fn rho2(&self, _: u32, _: Vec2, _: Vec2) -> Spectrum {
        self.r
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

    fn apply(&self, normal: Vec3, _: Vec3, from: Vec3) -> Spectrum {
        self.t * normal.dot(from).max(0.0) / std::f32::consts::PI
    }

    fn apply_sample(&self, _: Vec3, _: Vec3, _: Vec3, _: Vec2, _: f32, _: BxDFType) -> Spectrum {
        unimplemented!()
    }

    fn rho(&self, _: Vec3, _: u32, _: Vec2) -> Spectrum {
        self.t
    }

    fn rho2(&self, _: u32, _: Vec2, _: Vec2) -> Spectrum {
        self.t
    }
}
