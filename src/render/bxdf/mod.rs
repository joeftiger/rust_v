pub mod bsdf;
pub mod fresnel;
pub mod lambertian;
pub mod microfacet;
pub mod oren_nayar;
pub mod sampling;
pub mod specular;

use util::floats;

use crate::render::bxdf::sampling::cos_sample_hemisphere;
use crate::Spectrum;
use bitflags::_core::fmt::Debug;
use std::f32::consts::FRAC_1_PI;
use ultraviolet::{Rotor3, Vec2, Vec3};

#[inline(always)]
pub fn bxdf_normal() -> Vec3 {
    Vec3::unit_y()
}

#[inline(always)]
pub fn bxdf_incident_to(v: &Vec3) -> Vec3 {
    Vec3::new(-v.x, v.y, -v.z)
}

#[inline(always)]
pub fn is_neg(v: &Vec3) -> bool {
    v.y < 0.0
}

#[inline(always)]
pub fn flip_if_neg(mut v: Vec3) -> Vec3 {
    if is_neg(&v) {
        v.y = -v.y;
    }
    v
}

#[inline(always)]
pub fn bxdf_is_parallel(v: &Vec3) -> bool {
    v.y == 0.0
}

#[inline(always)]
pub fn cos_theta(v: &Vec3) -> f32 {
    v.y
}

#[inline(always)]
pub fn cos2_theta(v: &Vec3) -> f32 {
    cos_theta(v) * cos_theta(v)
}

#[inline(always)]
pub fn sin2_theta(v: &Vec3) -> f32 {
    f32::max(0.0, 1.0 - cos2_theta(v))
}

#[inline(always)]
pub fn sin_theta(v: &Vec3) -> f32 {
    sin2_theta(v).sqrt()
}

#[inline(always)]
pub fn tan_theta(v: &Vec3) -> f32 {
    sin_theta(v) / cos_theta(v)
}

#[inline(always)]
pub fn tan2_theta(v: &Vec3) -> f32 {
    sin2_theta(v) / cos2_theta(v)
}

#[inline(always)]
pub fn cos_phi(v: &Vec3) -> f32 {
    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        floats::fast_clamp(v.x / sin_theta, -1.0, 1.0)
    }
}

#[inline(always)]
pub fn sin_phi(v: &Vec3) -> f32 {
    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        floats::fast_clamp(v.z / sin_theta, -1.0, 1.0)
    }
}

#[inline(always)]
pub fn cos2_phi(v: &Vec3) -> f32 {
    let cos_phi = cos_phi(v);
    cos_phi * cos_phi
}

#[inline(always)]
pub fn sin2_phi(v: &Vec3) -> f32 {
    let sin_phi = sin_phi(v);
    sin_phi * sin_phi
}

#[inline(always)]
pub fn cos_d_phi(a: &Vec3, b: &Vec3) -> f32 {
    let abxz = a.x * b.x + a.z * b.z;
    let axz = a.x * a.x + a.z * a.z;
    let bxz = b.x * b.x + b.z * b.z;
    floats::fast_clamp(abxz / f32::sqrt(axz * bxz), -1.0, 1.0)
}

#[inline(always)]
pub fn same_hemisphere(a: &Vec3, b: &Vec3) -> bool {
    a.y * b.y > 0.0
}

#[inline(always)]
pub fn world_to_bxdf(v: &Vec3) -> Rotor3 {
    if *v != Vec3::unit_y() && *v != -Vec3::unit_y() {
        Rotor3::from_rotation_between(*v, Vec3::unit_y())
    } else {
        Rotor3::default()
    }
}

bitflags! {
    pub struct BxDFType: u8 {
        const NONE = 1 << 0;
        const REFLECTION = 1 << 1;
        const TRANSMISSION = 1 << 2;
        const DIFFUSE = 1 << 3;
        const GLOSSY = 1 << 4;
        const SPECULAR = 1 << 5;
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

/// # Summary
/// Contains of
/// * `spectrum` - An evaluated scaling spectrum
/// * `incident` - An evaluated incident direction
/// * `pdf` - An evaluated pdf
pub struct BxDFSample {
    pub spectrum: Spectrum,
    pub incident: Vec3,
    pub pdf: f32,
    pub typ: BxDFType,
}

impl BxDFSample {
    pub fn new(spectrum: Spectrum, incident: Vec3, pdf: f32, typ: BxDFType) -> Self {
        Self {
            spectrum,
            incident,
            pdf,
            typ,
        }
    }

    pub fn black_nan_0() -> Self {
        Self::new(0.0.into(), Vec3::broadcast(f32::NAN), 0.0, BxDFType::NONE)
    }
}

/// # Summary
/// The common base shared between BRDFs and BTDFs.
/// Provides methods for evaluating and sampling the distribution function for pairs of directions
/// at an intersection
pub trait BxDF: Debug + Send + Sync {
    /// # Summary
    /// Some light transport algorithms need to distinguish different BxDFTypes.
    ///
    /// # Results
    /// `BxDFType` - The type of this BxDF
    fn get_type(&self) -> BxDFType;

    /// # Summary
    /// Allows matching the user-supplied type to this BxDF.
    ///
    /// # Results
    /// * `bool` - Whether the type matches
    fn is_type(&self, t: BxDFType) -> bool {
        let st = self.get_type();
        (st & t) == st
    }

    /// # Summary
    /// Evaluates the BxDF for the pair of incident and outgoing light directions and the
    /// intersection normal.
    ///
    /// # Arguments
    /// * `normal` - The normal vector at the intersection
    /// * `incident` - The incident direction onto the intersection we evaluate
    /// * `outgoing` - The outgoing light direction
    ///
    /// # Results
    /// * `Spectrum` - The scaling spectrum at the intersection
    fn evaluate(&self, incident: &Vec3, outgoing: &Vec3) -> Spectrum;

    /// # Summary
    /// Samples an incident light direction for an outgoing light direction from the given sample
    /// space.
    ///
    /// # Arguments
    /// * `normal` - The normal vector at the intersection
    /// * `outgoing` - The outgoing light direction
    /// * `sample` - The sample space for randomization
    ///
    /// # Results
    /// * `BxDFSample` - The spectrum, incident and pdf at the intersection
    fn sample(&self, outgoing: &Vec3, sample: &Vec2) -> BxDFSample {
        let incident = cos_sample_hemisphere(sample);
        let incident = flip_if_neg(incident);

        let spectrum = self.evaluate(&incident, outgoing);
        let pdf = self.pdf(&incident, outgoing);

        BxDFSample::new(spectrum, incident, pdf, self.get_type())
    }

    /// # Summary
    /// Computes the probability density function (_pdf_) for the pair of directions.
    ///
    /// # Arguments
    /// * `normal` - The normal vector at the intersection
    /// * `incident` - The incident direction onto the intersection we evaluate
    /// * `outgoing` - The outgoing light direction
    ///
    /// # Results
    /// * `f32` - The evaluated pdf
    fn pdf(&self, incident: &Vec3, outgoing: &Vec3) -> f32 {
        if same_hemisphere(incident, outgoing) {
            cos_theta(incident).abs() * FRAC_1_PI
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub struct ScaledBxDF {
    bxdf: Box<dyn BxDF>,
    scale: Spectrum,
}

/// A scaled BxDF
///
impl ScaledBxDF {
    pub fn new(bxdf: Box<dyn BxDF>, scale: Spectrum) -> Self {
        Self { bxdf, scale }
    }
}

impl BxDF for ScaledBxDF {
    fn get_type(&self) -> BxDFType {
        self.bxdf.get_type()
    }

    fn evaluate(&self, view: &Vec3, from: &Vec3) -> Spectrum {
        self.scale * self.bxdf.evaluate(view, from)
    }

    fn sample(&self, outgoing: &Vec3, sample: &Vec2) -> BxDFSample {
        let mut sample = self.bxdf.sample(outgoing, sample);
        sample.spectrum *= self.scale;
        sample
    }

    fn pdf(&self, incident: &Vec3, outgoing: &Vec3) -> f32 {
        self.bxdf.pdf(incident, outgoing)
    }
}
