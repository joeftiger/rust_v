pub const DEFAULT_EPSILON: f32 = 10.0 * f32::EPSILON;

#[inline(always)]
#[must_use]
pub fn fast_min(a: f32, b: f32) -> f32 {
    if b < a {
        b
    } else {
        a
    }
}

#[inline(always)]
#[must_use]
pub fn fast_max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

#[inline(always)]
#[must_use]
pub fn approx_zero_tolerance(value: f32, zero_tolerance: f32) -> bool {
    value.abs() <= zero_tolerance
}

#[inline(always)]
#[must_use]
pub fn approx_zero(value: f32) -> bool {
    approx_zero_tolerance(value, DEFAULT_EPSILON)
}

#[inline(always)]
#[must_use]
pub fn approx_equal_tolerance(a: f32, b: f32, zero_tolerance: f32) -> bool {
    let distance = (b - a).abs();
    if distance <= zero_tolerance {
        true
    } else {
        let largest = fast_max(1.0, fast_max(a.abs(), b.abs()));
        distance <= largest * f32::EPSILON
    }
}

#[inline(always)]
#[must_use]
pub fn approx_equal(a: f32, b: f32) -> bool {
    approx_equal_tolerance(a, b, DEFAULT_EPSILON)
}

#[inline(always)]
#[must_use]
pub fn lt_epsilon_tolerance(a: f32, zero_tolerance: f32) -> bool {
    a < zero_tolerance
}

#[inline(always)]
#[must_use]
pub fn lt_epsilon(a: f32) -> bool {
    lt_epsilon_tolerance(a, DEFAULT_EPSILON)
}

#[inline(always)]
#[must_use]
pub fn fast_clamp(f: f32, min: f32, max: f32) -> f32 {
    fast_min(max, fast_max(min, f))
}