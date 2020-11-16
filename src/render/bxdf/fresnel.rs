use crate::color::Color;
use crate::Spectrum;

use crate::floats;
use crate::render::bxdf;
use crate::render::bxdf::{BxDF, BxDFSample, BxDFType};
use bitflags::_core::fmt::Debug;
use bitflags::_core::mem::swap;
use ultraviolet::{Vec2, Vec3};

#[inline(always)]
#[must_use]
pub fn dielectric_parallel(cos_i: f32, cos_t: f32, eta_i: f32, eta_t: f32) -> f32 {
    let it = eta_i * cos_t;
    let ti = eta_t * cos_i;

    (ti - it) / (ti + it)
}

#[inline(always)]
#[must_use]
pub fn dielectric_perpendicular(cos_i: f32, cos_t: f32, eta_i: f32, eta_t: f32) -> f32 {
    let tt = eta_t * cos_t;
    let ii = eta_i * cos_i;

    (ii - tt) / (ii + tt)
}

#[must_use]
pub fn fresnel_dielectric(mut cos_i: f32, mut eta_i: f32, mut eta_t: f32) -> f32 {
    cos_i = floats::fast_clamp(cos_i, -1.0, 1.0);
    // potentially swap indices of refraction
    let entering = cos_i > 0.0;
    if entering {
        swap(&mut eta_i, &mut eta_t);
        cos_i = cos_i.abs();
    }

    // compute cos_t using Snell's law
    let sin_i = floats::fast_max(0.0, 1.0 - cos_i * cos_i).sqrt();
    let sin_t = eta_i * sin_i / eta_t;

    // handle total internal reflection
    if sin_t >= 1.0 {
        return 1.0;
    }

    let cos_t = floats::fast_max(0.0, 1.0 - sin_t * sin_t).sqrt();
    let r_par = dielectric_parallel(cos_i, cos_t, eta_i, eta_t);
    let r_perp = dielectric_perpendicular(cos_i, cos_t, eta_i, eta_t);

    (r_par * r_par + r_perp * r_perp) / 2.0
}

#[must_use]
pub fn fresnel_conductor(
    mut cos_i: f32,
    eta_i: &Spectrum,
    eta_t: &Spectrum,
    k: &Spectrum,
) -> Spectrum {
    cos_i = floats::fast_clamp(cos_i, -1.0, 1.0);
    let eta = *eta_t / *eta_i;
    let etak = *k / *eta_i;

    let cos_i2 = cos_i * cos_i;
    let sin_i2 = 1.0 - cos_i2;
    let eta2 = eta * eta;
    let etak2 = etak * etak;

    let t0 = eta2 - etak2 - sin_i2.into();
    let a2_plus_b2 = (t0 * t0 + eta2 * etak2 * 4.0).sqrt();
    let t1 = a2_plus_b2 + cos_i2.into();
    let a = ((a2_plus_b2 + t0) * 0.5).sqrt();
    let t2 = a * (cos_i * 2.0);
    let r_s = (t1 - t2) / (t1 + t2);

    let t3 = a2_plus_b2 * cos_i2 + (sin_i2 * sin_i2).into();
    let t4 = t2 * sin_i2;
    let r_p = r_s * ((t3 - t4) / (t3 + t4));

    (r_p + r_s) / 2.0
}

pub trait Fresnel: Debug {
    fn evaluate(&self, cos_i: f32) -> Spectrum;
}

#[derive(Debug)]
pub struct Dielectric {
    pub eta_i: f32,
    pub eta_t: f32,
}

impl Dielectric {
    /// - `eta_t`: refractive index of material the light is entering.
    /// - `eta_i`: refractive index of material the light is coming from.
    pub fn new(eta_i: f32, eta_t: f32) -> Self {
        Self { eta_i, eta_t }
    }
}

impl Fresnel for Dielectric {
    fn evaluate(&self, cos_i: f32) -> Spectrum {
        fresnel_dielectric(cos_i, self.eta_i, self.eta_t).into()
    }
}

#[derive(Debug)]
pub struct Conductor {
    pub eta_t: Spectrum,
    pub eta_i: Spectrum,
    pub k: Spectrum,
}

impl Conductor {
    /// - `eta_t`: refractive index of material the light is entering.
    /// - `eta_i`: refractive index of material the light is coming from.
    pub fn new(eta_t: Spectrum, eta_i: Spectrum, absorption: Spectrum) -> Self {
        Self {
            eta_t,
            eta_i,
            k: absorption,
        }
    }
}

impl Fresnel for Conductor {
    fn evaluate(&self, cos_i: f32) -> Spectrum {
        fresnel_conductor(cos_i.abs(), &self.eta_i, &self.eta_t, &self.k)
    }
}

#[derive(Debug)]
pub struct FresnelNoOp;

impl Fresnel for FresnelNoOp {
    fn evaluate(&self, _: f32) -> Spectrum {
        Spectrum::white()
    }
}

#[derive(Debug)]
pub struct FresnelSpecular {
    r: Spectrum,
    t: Spectrum,
    fresnel: Dielectric,
}

impl FresnelSpecular {
    pub fn new(r: Spectrum, t: Spectrum, fresnel: Dielectric) -> Self {
        Self { r, t, fresnel }
    }
}

impl BxDF for FresnelSpecular {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::SPECULAR | BxDFType::TRANSMISSION
    }

    fn evaluate(&self, _: &Vec3, _: &Vec3) -> Spectrum {
        Spectrum::black()
    }

    fn sample(&self, outgoing: &Vec3, sample: &Vec2) -> BxDFSample {
        let f = fresnel_dielectric(
            bxdf::cos_theta(outgoing),
            self.fresnel.eta_i,
            self.fresnel.eta_t,
        );

        if sample.x < f {
            let incident = Vec3::new(-outgoing.x, -outgoing.y, outgoing.z);
            let pdf = f;
            let spectrum = self.r * (f / bxdf::cos_theta(&incident).abs());

            BxDFSample::new(spectrum, incident, pdf)
        } else {
            let entering = bxdf::cos_theta(outgoing) > 0.0;

            let (eta_i, eta_t, n) = if entering {
                (self.fresnel.eta_i, self.fresnel.eta_t, Vec3::unit_y())
            } else {
                (self.fresnel.eta_t, self.fresnel.eta_i, -Vec3::unit_y())
            };

            let incident = outgoing.refracted(n, eta_i / eta_t);
            let cos_i = bxdf::cos_theta(&incident);
            let pdf = 1.0 - f;
            let spectrum = self.t * (pdf / cos_i.abs());

            BxDFSample::new(spectrum, incident, pdf)
        }
    }
}
