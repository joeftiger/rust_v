use std::ops::{BitAnd, BitOr};
use ultraviolet::Vec3;
use crate::Spectrum;

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

const REFLECTION: BxDFType = BxDFType(1);
const TRANSMISSION: BxDFType = BxDFType(2);
const DIFFUSE: BxDFType = BxDFType(4);
const GLOSSY: BxDFType = BxDFType(8);
const SPECULAR: BxDFType = BxDFType(16);
#[allow(dead_code)]
const ALL: BxDFType = BxDFType(1 + 2 + 4 + 8 + 16);

pub trait BxDF: Send + Sync {
    fn get_type(&self) -> BxDFType;

    fn is_type(&self, t: BxDFType) -> bool {
        (self.get_type() & t) == t
    }

    fn apply(&self, view: Vec3, from: Vec3) -> Spectrum;

    // fn apply_sample(&self, view: Vec3, from: Vec3, )
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
