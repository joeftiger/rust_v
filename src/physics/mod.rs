use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use ultraviolet::{Mat3, Vec3};

use crate::physics::rgb::SRGB;
use crate::physics::xyz::XYZ;

pub mod cie;
pub mod light;
pub mod rgb;
pub mod xyz;

/// Returns the XYZ to sRGB matrix
pub fn xyz_to_srgb_mat() -> Mat3 {
    // https://en.wikipedia.org/wiki/SRGB#The_forward_transformation_(CIE_XYZ_to_sRGB)
    Mat3::new(
        Vec3::new(3.24096994, -0.96924364, 0.05563008),
        Vec3::new(-1.53738318, 1.8759675, -0.20397696),
        Vec3::new(-0.49861076, 0.04155506, 1.05697151)
    )
}

/// Returns the sRGB to XYZ matrix
pub fn srgb_to_xyz_mat() -> Mat3 {
    // https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
    Mat3::new(
        Vec3::new(0.41239080, 0.21263901, 0.01933082),
        Vec3::new(0.35758434, 0.71516868, 0.07219232),
        Vec3::new(0.18048079, 0.07219232, 0.95053215)
    )
}

/// Converts sRGB to linear
pub fn srgb_to_linear(val: f32) -> f32 {
    debug_assert!(val >= 0.0);
    debug_assert!(val <= 1.0);
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.040_448_237 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}

/// Converts sRGB to linear
pub fn srgbs_to_linear(val: Vec3) -> Vec3 {
    debug_assert!(val.component_min() >= 0.0);
    debug_assert!(val.component_max() <= 1.0);
    val.map(srgb_to_linear)
}

/// Converts linelar to sRGB
pub fn linear_to_srgb(val: f32) -> f32 {
    debug_assert!(val >= 0.0);
    debug_assert!(val <= 1.0);
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.003_130_668_5 {
        val * 12.92
    } else {
        1.055 * val.powf(1.0 / 2.4) - 0.055
    }
}

/// Converts linelar to sRGB
pub fn linears_to_srgb(val: Vec3) -> Vec3 {
    debug_assert!(val.component_min() >= 0.0);
    debug_assert!(val.component_max() <= 1.0);
    val.map(linear_to_srgb)
}

pub trait Color: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Mul<f32> + MulAssign<f32> + Div + DivAssign + Div<f32> + DivAssign<f32> + PartialEq + Index<usize> + IndexMut<usize> + Sized {

    /// Whether this color is black. Some computations can be omitted, if the color is black.
    fn is_black(&self) -> bool;

    /// Clamps the color values between min and max.
    fn clamp(&self, min: f32, max: f32) -> Self;

    /// Whether this color has NaN values.
    fn has_nans(&self) -> bool;

    /// Converts this color to sRGB.
    fn to_rgb(&self) -> SRGB;

    /// Converts this color to XYZ.
    fn to_xyz(&self) -> XYZ;
}

