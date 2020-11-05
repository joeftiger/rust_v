pub mod fresnel;
pub mod lambertian;
pub mod specular;
mod sampling;

use crate::Spectrum;
use ultraviolet::{Vec2, Vec3};
use std::f32::consts::PI;
use crate::render::bxdf::sampling::{sample_hemisphere, same_hemisphere};

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

/// # Summary
/// Contains of
/// * `spectrum` - An evaluated scaling spectrum
/// * `incident` - An evaluated incident direction
/// * `pdf` - An evaluated pdf
pub struct BxDFSample {
    pub spectrum: Spectrum,
    pub incident: Vec3,
    pub pdf: f32,
}

impl BxDFSample {
    pub fn new(spectrum: Spectrum, incident: Vec3, pdf: f32) -> Self {
        Self { spectrum, incident, pdf }
    }
}

/// # Summary
/// The common base shared between BRDFs and BTDFs.
/// Provides methods for evaluating and sampling the distribution function for pairs of directions
/// at an intersection
pub trait BxDF: Send + Sync {
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
        (self.get_type() & t) == t
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
    fn evaluate(&self, normal: Vec3, incident: Vec3, outgoing: Vec3) -> Spectrum;

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
    fn sample(
        &self,
        normal: Vec3,
        outgoing: Vec3,
        sample: Vec2,
    ) -> BxDFSample {
        let mut incident = sample_hemisphere(normal, sample);
        if !same_hemisphere(normal, incident, outgoing) {
            incident = -incident;
        }

        let spectrum = self.evaluate(normal, incident, outgoing);
        let pdf = self.pdf(normal, incident, outgoing);

        BxDFSample::new(spectrum, incident, pdf)
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
    fn pdf(&self, normal: Vec3, incident: Vec3, outgoing: Vec3) -> f32 {
        if same_hemisphere(normal, incident, outgoing) {
            normal.dot(incident).abs() * PI
        } else {
            0.0
        }
    }
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

    fn evaluate(&self, normal: Vec3, view: Vec3, from: Vec3) -> Spectrum {
        self.scale * self.bxdf.evaluate(normal, view, from)
    }

    fn sample(&self, normal: Vec3, outgoing: Vec3, sample: Vec2) -> BxDFSample {
        let mut sample = self.bxdf.sample(normal, outgoing, sample);
        sample.spectrum *= self.scale;
        sample
    }

    fn pdf(&self, normal: Vec3, incident: Vec3, outgoing: Vec3) -> f32 {
        self.bxdf.pdf(normal, incident, outgoing)
    }
}
