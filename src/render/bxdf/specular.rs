use crate::color::Color;
use crate::render::bxdf::fresnel::Fresnel;
use crate::render::bxdf::{BxDF, BxDFType};
use crate::Spectrum;
use ultraviolet::{Vec2, Vec3};

#[allow(dead_code)]
pub struct SpecularReflection {
    r: Spectrum,
    fresnel: Box<dyn Fresnel>,
}

impl SpecularReflection {
    pub fn new(r: Spectrum, fresnel: Box<dyn Fresnel>) -> Self {
        Self { r, fresnel }
    }
}

impl BxDF for SpecularReflection {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::SPECULAR
    }

    fn apply(&self, _: Vec3, _: Vec3, _: Vec3) -> Spectrum {
        Spectrum::black()
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

    #[allow(unused_variables)]
    fn rho(&self, w: Vec3, n_samples: u32, samples: Vec2) -> Spectrum {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn rho2(&self, n_samples: u32, samples1: Vec2, samples2: Vec2) -> Spectrum {
        unimplemented!()
    }
}
