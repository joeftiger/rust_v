use color::Color;
use crate::render::bxdf;
use crate::render::bxdf::fresnel::{Dielectric, Fresnel};
use crate::render::bxdf::{BxDF, BxDFSample, BxDFType};
use crate::Spectrum;
use std::rc::Rc;
use ultraviolet::{Vec2, Vec3};
use crate::render::bxdf::sampling::cos_sample_hemisphere;

#[derive(Debug)]
pub struct SpecularReflection {
    r: Spectrum,
    fresnel: Rc<dyn Fresnel>,
}

impl SpecularReflection {
    pub fn new(r: &Spectrum, fresnel: Rc<dyn Fresnel>) -> Self {
        Self { r: *r, fresnel }
    }
}

impl BxDF for SpecularReflection {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::SPECULAR
    }

    fn evaluate(&self, _incident: &Vec3, _outgoing: &Vec3) -> Spectrum {
        Spectrum::black()
    }

    fn sample(&self, outgoing: &Vec3, _sample: &Vec2) -> BxDFSample {
        let mut incident = Vec3::new(-outgoing.x, -outgoing.y, outgoing.z);
        // TODO: REMOVE THESE TWO LINES (originally not in there)
        incident += cos_sample_hemisphere(_sample) * _sample.component_min() / _sample.component_max();
        incident.normalize();

        let cos_i = bxdf::cos_theta(&incident);
        let spectrum = self.fresnel.evaluate(cos_i) * self.r / cos_i.abs();
        let pdf = 1.0;

        BxDFSample::new(spectrum, incident, pdf)
    }
}

#[derive(Debug)]
pub struct SpecularTransmission {
    t: Spectrum,
    fresnel: Rc<Dielectric>,
}

impl SpecularTransmission {
    pub fn new(t: &Spectrum, fresnel: Rc<Dielectric>) -> Self {
        Self { t: *t, fresnel }
    }
}

impl BxDF for SpecularTransmission {
    fn get_type(&self) -> BxDFType {
        BxDFType::SPECULAR | BxDFType::TRANSMISSION
    }

    fn evaluate(&self, _incident: &Vec3, _outgoing: &Vec3) -> Spectrum {
        Spectrum::black()
    }

    fn sample(&self, outgoing: &Vec3, _sample: &Vec2) -> BxDFSample {
        let entering = bxdf::cos_theta(outgoing) > 0.0;

        let (eta_i, eta_t, n) = if entering {
            (self.fresnel.eta_i, self.fresnel.eta_t, Vec3::unit_y())
        } else {
            (self.fresnel.eta_t, self.fresnel.eta_i, -Vec3::unit_y())
        };

        let incident = outgoing.refracted(n, eta_i / eta_t);
        let cos_i = bxdf::cos_theta(&incident);

        let f = Spectrum::new_const(1.0) - self.fresnel.evaluate(cos_i);
        let spectrum = f * self.t / cos_i.abs();

        BxDFSample::new(spectrum, incident, 1.0)
    }
}
