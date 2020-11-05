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

    fn evaluate(&self, _normal: Vec3, _incident: Vec3, _outgoing: Vec3) -> Spectrum {
        0.0.into()
    }

    fn sample(&self, normal: Vec3, outgoing: Vec3, _sample: Vec2) -> BxDFSample {
        let incident = Vec3::new(-outgoing.x, -outgoing.y, outgoing.z);
        let pdf = 1.0;
        let cos_theta = normal.dot(incident);
        let spectrum = self.fresnel.evaluate(cos_theta) * self.r / cos_theta.abs();

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

    fn evaluate(&self, _normal: Vec3, _incident: Vec3, _outgoing: Vec3) -> Spectrum {
        0.0.into()
    }

    fn sample(&self, normal: Vec3, outgoing: Vec3, _sample: Vec2) -> BxDFSample {
        let entering = normal.dot(outgoing) > 0.0;

        let (ei, et, n) = if entering {
            (self.fresnel.eta_i, self.fresnel.eta_t, normal)
        } else {
            (self.fresnel.eta_t, self.fresnel.eta_i, -normal)
        };

        let incident = outgoing.refracted(n, ei / et);
        let cos_theta = incident.dot(normal);
        let f: Spectrum = Spectrum::new_const(1.0) - self.fresnel.evaluate(cos_theta);
        let spectrum = f * self.t / cos_theta.abs();

        BxDFSample::new(spectrum, incident, 1.0)
    }
}
