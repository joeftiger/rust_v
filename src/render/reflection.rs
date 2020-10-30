use crate::{floats, Spectrum};
use crate::color::Color;
use bitflags::_core::mem::swap;

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
pub fn fresnel_conductor(mut cos_i: f32, eta_i: &Spectrum, eta_t: &Spectrum, k: &Spectrum) -> Spectrum {
    cos_i = floats::fast_clamp(cos_i, -1.0, 1.0);
    let eta = *eta_t / *eta_i;
    let etak = *k / *eta_i;

    let cos_i2 = cos_i * cos_i;
    let sin_i2 = 1.0 - cos_i2;
    let eta2 = eta * eta;
    let etak2 = etak * etak;

    let t0 = eta2 - etak2 - sin_i2.into();
    let a2_plus_b2 = (t0 * t0 + eta2 * etak2 * 4.0).sqrt();
    let t1 = a2_plus_b2 + Spectrum::new_const(cos_i2);
    let a = ((a2_plus_b2 + t0) * 0.5).sqrt();
    let t2 = a * (cos_i * 2.0);
    let r_s = (t1 - t2) / (t1 + t2);

    let t3 = a2_plus_b2 * cos_i2 + (sin_i2 * sin_i2).into();
    let t4 = t2 * sin_i2;
    let r_p = r_s * ((t3 - t4) / (t3 + t4));

    (r_p + r_s) / 2.0
}
