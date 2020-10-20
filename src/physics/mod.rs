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
    Mat3::from([3.2404542, -1.5371385, -0.4985314,
        -0.9692660, 1.8760108, 0.0415560,
        0.0556434, -0.2040259, 1.0572252])
}

/// Returns the sRGB to XYZ matrix
pub fn srgb_to_xyz_mat() -> Mat3 {
    Mat3::from([0.4124564, 0.3575761, 0.1804375,
        0.2126729, 0.7151522, 0.0721750,
        0.0193339, 0.1191920, 0.9503041])
}

/// Converts sRGB to linear
pub fn srgb_to_linear(val: f32) -> f32 {
    debug_assert!(val >= 0.0);
    debug_assert!(val <= 1.0);
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

