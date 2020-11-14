use crate::color::Color;
use crate::render::bxdf;
use crate::render::bxdf::fresnel::{Dielectric, Fresnel};
use crate::render::bxdf::{BxDF, BxDFSample, BxDFType};
use crate::Spectrum;
use ultraviolet::{Vec2, Vec3};

pub struct SpecularReflection<'a> {
    r: Spectrum,
    fresnel: &'a dyn Fresnel,
}

impl<'a> SpecularReflection<'a> {
    pub fn new(r: &Spectrum, fresnel: &'a dyn Fresnel) -> Self {
        Self { r: *r, fresnel }
    }
}

impl<'a> BxDF for SpecularReflection<'a> {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::SPECULAR
    }

    fn evaluate(&self, _incident: &Vec3, _outgoing: &Vec3) -> Spectrum {
        Spectrum::black()
    }

    fn sample(&self, outgoing: &Vec3, _sample: &Vec2) -> BxDFSample {
        let incident = Vec3::new(-outgoing.x, -outgoing.y, outgoing.z);

        let cos_i = bxdf::cos_theta(&incident);
        let spectrum = self.fresnel.evaluate(cos_i) * self.r / cos_i.abs();
        let pdf = 1.0;

        BxDFSample::new(spectrum, incident, pdf)
    }
}

pub struct SpecularTransmission<'a> {
    t: Spectrum,
    fresnel: &'a Dielectric,
}

impl<'a> SpecularTransmission<'a> {
    pub fn new(t: &Spectrum, fresnel: &'a Dielectric) -> Self {
        Self { t: *t, fresnel }
    }
}

impl<'a> BxDF for SpecularTransmission<'a> {
    fn get_type(&self) -> BxDFType {
        BxDFType::SPECULAR | BxDFType::TRANSMISSION
    }

    fn evaluate(&self, _incident: &Vec3, _outgoing: &Vec3) -> Spectrum {
        0.0.into()
    }

    fn sample(&self, outgoing: &Vec3, _sample: &Vec2) -> BxDFSample {
        let entering = bxdf::cos_theta(outgoing) > 0.0;

        let (ei, et, n) = if entering {
            (self.fresnel.eta_i, self.fresnel.eta_t, Vec3::unit_z())
        } else {
            (self.fresnel.eta_t, self.fresnel.eta_i, -Vec3::unit_z())
        };

        let incident = outgoing.refracted(n, ei / et);
        let cos_i = bxdf::cos_theta(&incident);

        let f = Spectrum::new_const(1.0) - self.fresnel.evaluate(cos_i);
        let spectrum = f * self.t / cos_i.abs();

        BxDFSample::new(spectrum, incident, 1.0)
    }
}
