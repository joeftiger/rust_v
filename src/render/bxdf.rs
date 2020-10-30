use crate::Spectrum;
use ultraviolet::Vec3;

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

    fn apply(&self, view: Vec3, from: Vec3) -> Spectrum;

    // fn apply_sample(&self, view: Vec3, from: Vec3, )
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

    fn apply(&self, view: Vec3, from: Vec3) -> Spectrum {
        self.scale * self.bxdf.apply(view, from)
    }
}

pub struct LambertianReflection(pub Spectrum);

impl BxDF for LambertianReflection {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::DIFFUSE
    }

    fn apply(&self, _: Vec3, _: Vec3) -> Spectrum {
        self.0 / std::f32::consts::PI
    }
}
