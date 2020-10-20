use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use ultraviolet::Mat3;

use crate::physics::rgb::SRGB;
use crate::physics::xyz::XYZ;

pub mod cie;
pub mod light;
pub mod rgb;
pub mod xyz;

pub fn xyz_to_srgb() -> Mat3 {
    Mat3::from([3.2404542, -1.5371385, -0.4985314,
        -0.9692660, 1.8760108, 0.0415560,
        0.0556434, -0.2040259, 1.0572252])
}

pub fn srgb_to_xyz() -> Mat3 {
    Mat3::from([0.4124564, 0.3575761, 0.1804375,
        0.2126729, 0.7151522, 0.0721750,
        0.0193339, 0.1191920, 0.9503041])
}

pub fn srgb_to_linear(val: f32) -> f32 {
    debug_assert!(val >= 0.0);
    debug_assert!(val <= 1.0);
    if val <= 0.040_448_237 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}

pub fn linear_to_srgb(val: f32) -> f32 {
    debug_assert!(val >= 0.0);
    debug_assert!(val <= 1.0);
    if val <= 0.003_130_668_5 {
        val * 12.92
    } else {
        1.055 * val.powf(1.0 / 2.4) - 0.055
    }
}

pub trait Color: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Mul<f32> + MulAssign<f32> + Div + DivAssign + Div<f32> + DivAssign<f32> + PartialEq + Index<usize> + IndexMut<usize> + Sized {
    fn is_black(&self) -> bool;

    fn clamp(&self, min: f32, max: f32) -> Self;

    fn has_nans(&self) -> bool;

    fn to_rgb(&self) -> SRGB;

    fn to_xyz(&self) -> XYZ;
}

