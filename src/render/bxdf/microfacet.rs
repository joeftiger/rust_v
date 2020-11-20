use std::f32::consts::{FRAC_PI_2, PI};

use ultraviolet::{Vec2, Vec3};

use color::Color;
use geometry::spherical_direction;
use util::{floats, math};

use crate::render::bxdf::fresnel::{Dielectric, Fresnel};
use crate::render::bxdf::*;
use crate::Spectrum;

#[allow(dead_code)]
pub fn roughness_to_alpha(roughness: f32) -> f32 {
    let roughness = roughness.max(floats::BIG_EPSILON);
    let x = roughness.ln();
    let x2 = x * x;

    1.62142 + 0.819955 * x + 0.1734 * x2 + 0.0171201 * x2 * x + 0.000640711 * x2 * x2
}

pub trait MicrofacetDistribution: Debug {
    fn d(&self, wh: &Vec3) -> f32;

    fn lambda(&self, w: &Vec3) -> f32;

    fn g1(&self, w: &Vec3) -> f32 {
        1.0 / (1.0 + self.lambda(w))
    }

    fn g(&self, wi: &Vec3, wo: &Vec3) -> f32 {
        1.0 / (1.0 + self.lambda(wo) + self.lambda(wi))
    }

    fn sample_wh(&self, wo: &Vec3, sample: &Vec2) -> Vec3;

    fn pdf(&self, wo: &Vec3, wh: &Vec3) -> f32 {
        if self.is_sample_visible_area() {
            self.d(wh) * self.g1(wo) * wo.dot(*wh).abs() / cos_theta(wo).abs()
        } else {
            self.d(wh) * cos_theta(wh).abs()
        }
    }

    fn is_sample_visible_area(&self) -> bool;
}

#[derive(Debug)]
pub struct BeckmannDistribution {
    alpha_x: f32,
    alpha_y: f32,
    sample_visible_area: bool,
}

impl BeckmannDistribution {
    pub fn new(alpha_x: f32, alpha_y: f32, sample_visible_area: bool) -> Self {
        Self {
            alpha_x,
            alpha_y,
            sample_visible_area,
        }
    }

    fn beckmann_sample11(cos_theta_i: f32, sample: &Vec2) -> Vec2 {
        /* Special case (normal incidence) */
        if cos_theta_i > 1.0 - floats::BIG_EPSILON {
            let r = f32::sqrt(-f32::ln(1.0 - sample.x));
            let phi = 2.0 * PI * sample.y;

            return Vec2::new(r * phi.sin(), r * phi.cos());
        }

        /* The original inversion routine from the paper contained
        discontinuities, which causes issues for QMC integration
        and techniques like Kelemen-style MLT. The following code
        performs a numerical inversion with better behavior */
        let sin_theta_i = 0f32.max(1.0 - cos_theta_i * cos_theta_i).sqrt();
        let tan_theta_i = sin_theta_i / cos_theta_i;
        let cot_theta_i = 1.0 / tan_theta_i;

        /* Search interval -- everything is parameterized
        in the Erf() domain */
        let mut a = -1.0;
        let mut c = math::erf(cot_theta_i);
        let sample_x = sample.x.max(floats::BIG_EPSILON);

        /* Start with a good initial guess */
        // Float b = (1-sample_x) * a + sample_x * c;

        /* We can do better (inverse of an approximation computed in
         * Mathematica) */
        let theta_i = cos_theta_i.acos();
        let fit = 1.0 + theta_i * (-0.876 + theta_i * (0.4265 - 0.0594 * theta_i));
        let mut b = c - (1.0 + c) * f32::powf(1.0 - sample_x, fit);

        /* Normalization factor for the CDF */

        let normalization = 1.0
            / (1.0
                + c
                + floats::FRAC_1_SQRT_PI * tan_theta_i * f32::exp(-cot_theta_i * cot_theta_i));

        for _ in 0..10 {
            /* Bisection criterion -- the oddly-looking
            Boolean expression are intentional to check
            for NaNs at little additional cost */
            // if !(b >= a && b <= c) {
            if b < a || b > c {
                b = 0.5 * (a + c);
            }

            /* Evaluate the CDF and its derivative
            (i.e. the density function) */
            let inv_erf = math::erf_inv(b);
            let value = normalization
                * (1.0 + b + floats::FRAC_1_SQRT_PI * tan_theta_i * f32::exp(-inv_erf * inv_erf))
                - sample_x;
            if value.abs() < floats::BIG_EPSILON {
                break;
            }

            let derivative = normalization * (1.0 - inv_erf * tan_theta_i);

            /* Update bisection intervals */
            if value > 0.0 {
                c = b;
            } else {
                a = b;
            }

            b -= value / derivative;
        }

        /* Now convert back into a slope value */
        let out = Vec2::new(
            math::erf_inv(b),
            math::erf_inv(2.0 * sample.y.max(floats::BIG_EPSILON) - 1.0),
        );

        debug_assert!(out.x.is_finite());
        debug_assert!(out.y.is_finite());

        out
    }

    fn beckmann_sample(wi: &Vec3, alpha_x: f32, alpha_y: f32, sample: &Vec2) -> Vec3 {
        // 1. stretch wi
        let wi_stretched = Vec3::new(alpha_x * wi.x, alpha_y * wi.y, wi.z).normalized();

        // 2. simulate P22_{wi}(x_slope, y_slope, 1, 1)
        let cos_theta = cos_theta(&wi_stretched);
        let mut slope = Self::beckmann_sample11(cos_theta, sample);

        // 3. rotate
        let cos_phi = cos_phi(&wi_stretched);
        let sin_phi = sin_phi(&wi_stretched);
        let tmp = cos_phi * slope.x - sin_phi * slope.y;
        slope.y = sin_phi * slope.x + cos_phi * slope.y;
        slope.x = tmp;

        // 4. unstretch
        slope.x *= alpha_x;
        slope.y *= alpha_y;

        // 5. compute normal
        Vec3::new(-slope.x, -slope.y, 1.0).normalized()
    }
}

impl MicrofacetDistribution for BeckmannDistribution {
    fn d(&self, wh: &Vec3) -> f32 {
        let tan2_theta = tan2_theta(wh);
        if tan2_theta.is_infinite() {
            0.0
        } else {
            let cos2_theta = cos2_theta(wh);
            let cos4_theta = cos2_theta * cos2_theta;

            let alpha_x2 = self.alpha_x * self.alpha_x;

            let cos2 = cos2_phi(wh) / alpha_x2;
            let sin2 = sin2_phi(wh) / self.alpha_y * self.alpha_y;

            f32::exp(-tan2_theta * (cos2 + sin2)) / (PI * alpha_x2 * cos4_theta)
        }
    }

    fn lambda(&self, w: &Vec3) -> f32 {
        let tan_theta = tan_theta(w);
        if tan_theta.is_infinite() {
            return 0.0;
        }

        // Compute _alpha_ for direction _w_
        let cos2 = cos2_phi(w) * self.alpha_x * self.alpha_x;
        let sin2 = sin2_phi(w) * self.alpha_y * self.alpha_y;

        let alpha = f32::sqrt(cos2 + sin2);
        let a = 1.0 / (alpha * tan_theta.abs());
        if a >= floats::BIG_EPSILON {
            0.0
        } else {
            (1.0 - 1.259 * a + 0.396 * a * a) / (3.535 * a + 2.181 * a * a)
        }
    }

    fn sample_wh(&self, wo: &Vec3, sample: &Vec2) -> Vec3 {
        if self.sample_visible_area {
            let is_neg = is_neg(wo);
            let wo_new = if is_neg { -*wo } else { *wo };

            let mut wh = Self::beckmann_sample(&wo_new, self.alpha_x, self.alpha_y, sample);
            if is_neg {
                wh = -wh;
            }

            return wh;
        }

        // Sample full distribution of normals for Beckmann distribution

        // Compute $\tan^2 \theta$ and $\phi$ for Beckmann distribution sample
        let alpha_x2 = self.alpha_x * self.alpha_x;
        let log_sample = f32::ln(1.0 - sample.x);
        debug_assert!(log_sample.is_finite());

        let tan2_theta: f32;
        let phi: f32;
        if sample.x == sample.y {
            tan2_theta = -alpha_x2 * log_sample;
            phi = floats::PI_2 * sample.y;
        } else {
            // Compute _tan2Theta_ and _phi_ for anisotropic Beckmann
            // distribution
            let tan = f32::tan(floats::PI_2 * sample.y * FRAC_PI_2);
            let mut phi_new = f32::atan(self.alpha_y * tan / self.alpha_x);
            if sample.y > 0.5 {
                phi_new += PI;
            }
            phi = phi_new;

            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            tan2_theta = -log_sample
                / (cos_phi * cos_phi / alpha_x2
                    + sin_phi * sin_phi / (self.alpha_y * self.alpha_y));
        }

        // Map sampled Beckmann angles to normal direction _wh_
        let cos_theta = 1.0 / f32::sqrt(1.0 - tan2_theta);
        let sin_theta = 0f32.max(1.0 - cos_theta * cos_theta).sqrt();

        let mut wh = spherical_direction(sin_theta, cos_theta, phi);
        if !same_hemisphere(wo, &wh) {
            wh = -wh;
        }

        wh
    }

    fn is_sample_visible_area(&self) -> bool {
        self.sample_visible_area
    }
}

// pub struct TrowbridgeReitzDistribution {
//     alpha_x: f32,
//     alpha_y: f32,
//     sample_visible_area: bool
// }
//
// impl TrowbridgeReitzDistribution {
//     pub fn new(alpha_x: f32, alpha_y: f32, sample_visible_area: bool) -> Self {
//         Self { alpha_x, alpha_y, sample_visible_area }
//     }
// }
//
// impl MicrofacetDistribution for TrowbridgeReitzDistribution {
//     fn d(&self, wh: &Vec3) -> f32 {
//         unimplemented!()
//     }
//
//     fn lambda(&self, w: &Vec3) -> f32 {
//         let tan_theta = tan_theta(w);
//         if tan_theta.is_infinite() {
//             return 0.0;
//         }
//
//         let tan2 = tan_theta * tan_theta;
//         let cos2 = cos2_phi(w) * self.alpha_x * self.alpha_x;
//         let sin2 = sin2_phi(w) * self.alpha_y * self.alpha_y;
//
//         let alpha = f32::sqrt(cos2 + sin2);
//         let alpha2_tan2 = tan2 * alpha * alpha;
//
//         (-1.0 + f32::sqrt(1.0 + alpha2_tan2)) / 2.0
//     }
//
//     fn sample_wh(&self, wo: &Vec3, sample: &Vec2) -> Vec3 {
//         unimplemented!()
//     }
//
//     fn pdf(&self, wo: &Vec3, wh: &Vec3) -> f32 {
//         unimplemented!()
//     }
// }

#[derive(Debug)]
pub struct MicrofacetReflection {
    r: Spectrum,
    distribution: Box<dyn MicrofacetDistribution>,
    fresnel: Box<dyn Fresnel>,
}

impl MicrofacetReflection {
    pub fn new(
        r: Spectrum,
        distribution: Box<dyn MicrofacetDistribution>,
        fresnel: Box<dyn Fresnel>,
    ) -> Self {
        Self {
            r,
            distribution,
            fresnel,
        }
    }
}

impl BxDF for MicrofacetReflection {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::GLOSSY
    }

    fn evaluate(&self, incident: &Vec3, outgoing: &Vec3) -> Spectrum {
        let cos_theta_i = cos_theta(incident).abs();
        let cos_theta_o = cos_theta(outgoing).abs();
        if cos_theta_i == 0.0 || cos_theta_o == 0.0 {
            Spectrum::black();
        }

        let wh = *incident + *outgoing;
        if wh == Vec3::zero() {
            Spectrum::black();
        }

        let wh = wh.normalized();

        let f = self.fresnel.evaluate(incident.dot(wh));
        let dist = self.r * self.distribution.d(&wh) * self.distribution.g(incident, outgoing);

        dist * f / (4.0 * cos_theta_i * cos_theta_o)
    }

    fn sample(&self, outgoing: &Vec3, sample: &Vec2) -> BxDFSample {
        // Sample microfacet orientation $\wh$ and reflected direction $\wi$
        if bxdf_is_parallel(outgoing) {
            return BxDFSample::black_nan_0();
        }

        let wh = self.distribution.sample_wh(outgoing, sample);
        let cos_o = outgoing.dot(wh);
        // Should be rare
        if cos_o < 0.0 {
            return BxDFSample::black_nan_0();
        }

        let incident = outgoing.reflected(wh);
        if !same_hemisphere(&incident, outgoing) {
            return BxDFSample::black_nan_0();
        }

        let spectrum = self.evaluate(&incident, outgoing);
        let pdf = self.distribution.pdf(outgoing, &wh) / (4.0 * cos_o);

        BxDFSample::new(spectrum, incident, pdf)
    }

    fn pdf(&self, incident: &Vec3, outgoing: &Vec3) -> f32 {
        if !same_hemisphere(incident, outgoing) {
            0.0
        } else {
            let wh = (*incident + *outgoing).normalized();

            self.distribution.pdf(outgoing, &wh) / (4.0 * outgoing.dot(wh))
        }
    }
}

#[derive(Debug)]
pub struct MicrofacetTransmission {
    t: Spectrum,
    distribution: Box<dyn MicrofacetDistribution>,
    fresnel: Dielectric,
}

impl MicrofacetTransmission {
    pub fn new(
        t: Spectrum,
        distribution: Box<dyn MicrofacetDistribution>,
        fresnel: Dielectric,
    ) -> Self {
        Self {
            t,
            distribution,
            fresnel,
        }
    }
}

impl BxDF for MicrofacetTransmission {
    fn get_type(&self) -> BxDFType {
        BxDFType::TRANSMISSION | BxDFType::GLOSSY
    }

    fn evaluate(&self, incident: &Vec3, outgoing: &Vec3) -> Spectrum {
        if same_hemisphere(incident, outgoing) {
            return Spectrum::black();
        }

        let cos_theta_i = cos_theta(incident);
        let cos_theta_o = cos_theta(outgoing);
        if cos_theta_i == 0.0 || cos_theta_o == 0.0 {
            return Spectrum::black();
        }

        // Compute $\wh$ from $\wo$ and $\wi$ for microfacet transmission
        let (eta, wh) = {
            let eta = if cos_theta_o > 0.0 {
                self.fresnel.eta_t / self.fresnel.eta_i
            } else {
                self.fresnel.eta_i / self.fresnel.eta_t
            };
            let wh = flip_if_neg(*outgoing + *incident * eta);
            (eta, wh)
        };

        // Same side?
        let cos_i = outgoing.dot(wh);
        let cos_t = incident.dot(wh);
        if cos_i * cos_t > 0.0 {
            return Spectrum::black();
        }

        let f = self.fresnel.evaluate(cos_i);

        let sqrt_denom = cos_i + eta * cos_t;

        let t = (Spectrum::new_const(1.0) - f) * self.t;
        let dist = self.distribution.d(&wh) * self.distribution.g(incident, outgoing);
        let factor =
            cos_i.abs() * cos_t.abs() / (cos_theta_i * cos_theta_i * sqrt_denom * sqrt_denom);

        t * (dist * factor).abs()
    }

    fn sample(&self, outgoing: &Vec3, sample: &Vec2) -> BxDFSample {
        if bxdf_is_parallel(outgoing) {
            return BxDFSample::black_nan_0();
        }

        let wh = self.distribution.sample_wh(outgoing, sample);
        // Should be rare
        if outgoing.dot(wh) < 0.0 {
            return BxDFSample::black_nan_0();
        }

        let eta = if cos_theta(outgoing) > 0.0 {
            self.fresnel.eta_i / self.fresnel.eta_t
        } else {
            self.fresnel.eta_t / self.fresnel.eta_i
        };

        let incident = outgoing.refracted(wh, eta);
        let spectrum = self.evaluate(&incident, outgoing);
        let pdf = self.pdf(&incident, outgoing);

        BxDFSample::new(spectrum, incident, pdf)
    }

    fn pdf(&self, incident: &Vec3, outgoing: &Vec3) -> f32 {
        if same_hemisphere(incident, outgoing) {
            return 0.0;
        }

        // Compute $\wh$ from $\wo$ and $\wi$ for microfacet transmission
        let eta = if cos_theta(outgoing) > 0.0 {
            self.fresnel.eta_i / self.fresnel.eta_t
        } else {
            self.fresnel.eta_t / self.fresnel.eta_i
        };

        let wh = (*outgoing + *incident * eta).normalized();

        let cos_i = incident.dot(wh);
        let cos_o = outgoing.dot(wh);
        if cos_i * cos_o > 0.0 {
            return 0.0;
        }

        let sqrt_denom = cos_o + eta * cos_i;
        let dwh_dwi = eta * eta * cos_i.abs() / (sqrt_denom * sqrt_denom);

        self.distribution.pdf(outgoing, &wh) * dwh_dwi
    }
}
