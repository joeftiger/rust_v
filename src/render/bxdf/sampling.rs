use ultraviolet::{Vec2, Vec3, Rotor3};
use std::f32::consts::*;
use crate::geometry::AngularExt;

pub fn same_hemisphere(normal: Vec3, a: Vec3, b: Vec3) -> bool {
    normal.angle_to(&a) <= PI && normal.angle_to(&b) <= PI
}

/// # Summary
/// Samples a concentric mapped point from the given random sample.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec2` - A concentric sample
pub fn concentric_sample_disk(sample: Vec2) -> Vec2 {
    debug_assert!(sample.x >= 0.0);
    debug_assert!(sample.x <= 1.0);
    debug_assert!(sample.y >= 0.0);
    debug_assert!(sample.y <= 1.0);

    // Map uniform random numbers to [-1,1]^2
    let offset = 2.0 * sample - Vec2::one();

    // Handle degeneracy at the origin
    if offset.x == 0.0 || offset.y == 0.0 {
        return Vec2::zero();
    }

    // Apply concentric mapping to point
    let theta;
    let r;
    if offset.x.abs() > offset.y.abs() {
        theta = FRAC_PI_4 * offset.y / offset.x;
        r = offset.x;
    } else {
        theta = FRAC_PI_2 - FRAC_PI_4 * offset.x / offset.y;
        r = offset.y;
    }

    r * Vec2::new(theta.cos(), theta.sin())
}

/// # Summary
/// Samples a point in a hemisphere described by the sample.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec3` - A point on the hemisphere
pub fn sample_hemisphere(normal: Vec3, sample: Vec2) -> Vec3 {
    debug_assert!(sample.x >= 0.0);
    debug_assert!(sample.x <= 1.0);
    debug_assert!(sample.y >= 0.0);
    debug_assert!(sample.y <= 1.0);

    // TODO: Make more efficient
    let rotation = Rotor3::from_rotation_between(Vec3::unit_z(), normal);

    let d = concentric_sample_disk(sample);
    let z = f32::sqrt(f32::max(0.0, 1.0 - d.x * d.x - d.y * d.y));

    (rotation * Vec3::new(d.x, d.y, z)).normalized()
}
