use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use ultraviolet::{Vec2, Vec3};

/// # Summary
/// Samples a concentric mapped point from the given random sample.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec2` - A concentric sample
pub fn concentric_sample_disk(sample: &Vec2) -> Vec2 {
    debug_assert!(sample.x >= 0.0);
    debug_assert!(sample.x < 1.0);
    debug_assert!(sample.y >= 0.0);
    debug_assert!(sample.y < 1.0);

    // Map uniform random numbers to [-1,1]^2
    let offset = 2.0 * *sample - Vec2::one();

    // Handle degeneracy at the origin
    if offset.x == 0.0 || offset.y == 0.0 {
        return Vec2::zero();
    }

    // Apply concentric mapping to point
    let r;
    let theta;
    if offset.x.abs() > offset.y.abs() {
        r = offset.x;
        theta = FRAC_PI_4 * offset.y / offset.x;
    } else {
        r = offset.y;
        theta = FRAC_PI_2 - FRAC_PI_4 * offset.x / offset.y;
    }

    r * Vec2::new(theta.cos(), theta.sin())
}

/// # Summary
/// Samples a hemisphere with a cosine distribution described by the sample.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec3` - A point on the hemisphere around `(0, 0, 1)`
pub fn cos_sample_hemisphere(sample: &Vec2) -> Vec3 {
    let d = concentric_sample_disk(sample);
    let z = 0.0f32.max(1.0 - d.x * d.x - d.y * d.y).sqrt();

    Vec3::new(d.x, d.y, z)
}
