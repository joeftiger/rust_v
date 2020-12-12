use crate::floats;

/// # Summary
/// Solves a quadratic equation, handling generics.
///
/// # Arguments
/// `a`x^2 + `b`x + `c`
///
/// # Returns
/// * `Option<(f32, f32)>` - The solutions in ascending order (if any)
#[inline(always)]
#[must_use]
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    debug_assert!(!a.is_nan());
    debug_assert!(!b.is_nan());
    debug_assert!(!c.is_nan());

    if floats::lt_epsilon(a) {
        if floats::lt_epsilon(b) {
            return None;
        }

        let sol = -c / b;

        return Some((sol, sol));
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let a_x1 = -0.5 * (b + f32::copysign(discriminant.sqrt(), b));

    let x0 = a_x1 / a;
    let x1 = c / a_x1;

    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

#[allow(clippy::excessive_precision)]
#[inline(always)]
#[must_use]
pub fn erf(x: f32) -> f32 {
    // constants
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    // Save the sign of x
    let sign = x.signum();
    let x = x.abs();

    // A&S formula 7.1.26
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * f32::exp(-x * x);

    sign * y
}

#[allow(clippy::excessive_precision)]
#[inline]
pub fn erf_inv(x: f32) -> f32 {
    let x = floats::fast_clamp(x, -1.0 + floats::BIG_EPSILON, 1.0 - floats::BIG_EPSILON);

    let mut w = -f32::ln((1.0 - x) * (1.0 + x));
    let mut p: f32;
    if w < 5.0 {
        w -= 2.5;
        p = 2.81022636e-08;
        p = 3.43273939e-07 + p * w;
        p = -3.5233877e-06 + p * w;
        p = -4.39150654e-06 + p * w;
        p = 0.00021858087 + p * w;
        p = -0.00125372503 + p * w;
        p = -0.00417768164 + p * w;
        p = 0.246640727 + p * w;
        p = 1.50140941 + p * w;
    } else {
        w = w.sqrt() - 3.0;
        p = -0.000200214257;
        p = 0.000100950558 + p * w;
        p = 0.00134934322 + p * w;
        p = -0.00367342844 + p * w;
        p = 0.00573950773 + p * w;
        p = -0.0076224613 + p * w;
        p = 0.00943887047 + p * w;
        p = 1.00167406 + p * w;
        p = 2.83297682 + p * w;
    }

    p * x
}
