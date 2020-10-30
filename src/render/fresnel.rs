use crate::Spectrum;
use crate::render::reflection;

pub trait Fresnel {
    fn fresnel(&self, cos_i: f32) -> f32;
}

pub struct Dielectric {
    pub eta_t: f32,
    pub eta_i: f32,
}

impl Dielectric {
    /// - `eta_t`: refractive index of material the light is entering.
    /// - `eta_i`: refractive index of material the light is coming from.
    pub fn new(eta_t: f32, eta_i: f32) -> Self {
        Self { eta_t, eta_i }
    }
}

impl Fresnel for Dielectric {
    fn fresnel(&self, cos_i: f32) -> f32 {
        reflection::fresnel_dielectric(cos_i, self.eta_i, self.eta_t)
    }
}

pub struct Conductor {
    pub eta_t: Spectrum,
    pub eta_i: Spectrum,
    pub k: Spectrum
}

impl Conductor {
    /// - `eta_t`: refractive index of material the light is entering.
    /// - `eta_i`: refractive index of material the light is coming from.
    pub fn new(eta_t: Spectrum, eta_i: Spectrum, absorption: Spectrum) -> Self {
        Self { eta_t, eta_i, k: absorption }
    }
}

// impl Fresnel for Conductor {
//     fn fresnel(&self, cos_i: f32) -> f32 {
//         fresnel_conductor(cos_i.abs(), &self.eta_i, &self.eta_t, &self.k)
//     }
// }
