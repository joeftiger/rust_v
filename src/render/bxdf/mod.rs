pub mod fresnel;
pub mod lambertian;
pub mod reflection;
pub mod specular;

use crate::Spectrum;
use ultraviolet::{Vec2, Vec3};

bitflags! {
    pub struct BxDFType: u8 {
        const REFLECTION = 1 << 0;
        const TRANSMISSION = 1 << 1;
        const DIFFUSE = 1 << 2;
        const GLOSSY = 1 << 3;
        const SPECULAR = 1 << 4;
        const ALL = Self::REFLECTION.bits | Self::TRANSMISSION.bits | Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits;
    }
}

impl BxDFType {
    pub fn is_reflection(&self) -> bool {
        *self & Self::REFLECTION == Self::REFLECTION
    }

    pub fn is_transmission(&self) -> bool {
        *self & Self::TRANSMISSION == Self::TRANSMISSION
    }

    pub fn is_diffuse(&self) -> bool {
        *self & Self::DIFFUSE == Self::DIFFUSE
    }

    pub fn is_glossy(&self) -> bool {
        *self & Self::GLOSSY == Self::GLOSSY
    }

    pub fn is_specular(&self) -> bool {
        *self & Self::SPECULAR == Self::SPECULAR
    }
}

pub trait BxDF: Send + Sync {
    fn get_type(&self) -> BxDFType;

    fn is_type(&self, t: BxDFType) -> bool {
        (self.get_type() & t) == t
    }

    fn apply(&self, normal: Vec3, view: Vec3, from: Vec3) -> Spectrum;

    fn apply_sample(
        &self,
        normal: Vec3,
        view: Vec3,
        from: Vec3,
        sample: Vec2,
        pdf: f32,
        sampled_type: BxDFType,
    ) -> Spectrum;

    fn rho(&self, w: Vec3, n_samples: u32, samples: Vec2) -> Spectrum;

    fn rho2(&self, n_samples: u32, samples1: Vec2, samples2: Vec2) -> Spectrum;
}

pub struct ScaledBxDF {
    bxdf: Box<dyn BxDF>,
    scale: Spectrum,
}

/// A scaled BxDF
impl ScaledBxDF {
    pub fn new(bxdf: Box<dyn BxDF>, scale: Spectrum) -> Self {
        Self { bxdf, scale }
    }
}

impl BxDF for ScaledBxDF {
    fn get_type(&self) -> BxDFType {
        self.bxdf.get_type()
    }

    fn apply(&self, normal: Vec3, view: Vec3, from: Vec3) -> Spectrum {
        self.scale * self.bxdf.apply(normal, view, from)
    }

    fn apply_sample(
        &self,
        normal: Vec3,
        view: Vec3,
        from: Vec3,
        sample: Vec2,
        pdf: f32,
        sampled_type: BxDFType,
    ) -> Spectrum {
        self.scale
            * self
                .bxdf
                .apply_sample(normal, view, from, sample, pdf, sampled_type)
    }

    fn rho(&self, w: Vec3, n_samples: u32, samples: Vec2) -> Spectrum {
        self.scale * self.bxdf.rho(w, n_samples, samples)
    }

    fn rho2(&self, n_samples: u32, samples1: Vec2, samples2: Vec2) -> Spectrum {
        self.scale * self.bxdf.rho2(n_samples, samples1, samples2)
    }
}
