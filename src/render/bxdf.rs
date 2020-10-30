use std::ops::{BitAnd, BitOr};
use ultraviolet::Vec3;
use crate::Spectrum;

const REFLECTION: BxDFType = BxDFType(1 << 0);
const TRANSMISSION: BxDFType = BxDFType(1 << 1);
const DIFFUSE: BxDFType = BxDFType(1 << 2);
const GLOSSY: BxDFType = BxDFType(1 << 3);
const SPECULAR: BxDFType = BxDFType(1 << 4);
#[allow(dead_code)]
const ALL: BxDFType = BxDFType(REFLECTION.0 | TRANSMISSION.0 | DIFFUSE.0 | GLOSSY.0 | SPECULAR.0);

#[derive(Copy, Clone)]
pub struct BxDFType(u8);

impl BxDFType {
    pub fn is_reflection(&self) -> bool {
        *self & REFLECTION == REFLECTION
    }

    pub fn is_transmission(&self) -> bool {
        *self & TRANSMISSION == TRANSMISSION
    }

    pub fn is_diffuse(&self) -> bool {
        *self & DIFFUSE == DIFFUSE
    }

    pub fn is_glossy(&self) -> bool {
        *self & GLOSSY == GLOSSY
    }

    pub fn is_specular(&self) -> bool {
        *self & SPECULAR == SPECULAR
    }
}

impl BitOr for BxDFType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BxDFType(self.0 | rhs.0)
    }
}

impl BitAnd for BxDFType {
    type Output = BxDFType;

    fn bitand(self, rhs: Self) -> Self::Output {
        BxDFType(self.0 & rhs.0)
    }
}

impl PartialEq for BxDFType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub trait BxDF: Send + Sync {
    fn get_type(&self) -> BxDFType;

    fn is_type(&self, t: BxDFType) -> bool {
        (self.get_type() & t) == t
    }

    fn apply(&self, view: Vec3, from: Vec3) -> Spectrum;

    // fn apply_sample(&self, view: Vec3, from: Vec3, )
}

pub struct ScaledBxDF {
    bxdf: Box<dyn BxDF>,
    scale: Spectrum
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

    fn apply(&self, view: Vec3, from: Vec3) -> Spectrum {
        self.scale * self.bxdf.apply(view, from)
    }
}

pub struct LambertianReflection(pub Spectrum);

impl BxDF for LambertianReflection {
    fn get_type(&self) -> BxDFType {
        REFLECTION | DIFFUSE
    }

    fn apply(&self, _: Vec3, _: Vec3) -> Spectrum{
        self.0 / std::f32::consts::PI
    }
}
