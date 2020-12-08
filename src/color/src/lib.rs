use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use util::floats;
use image::Rgb;
use std::fmt::Debug;
use ultraviolet::{Mat3, Vec3};

pub mod cie;
pub mod spectral;
pub mod srgb;
pub mod xyz;

pub use spectral::*;
pub use srgb::*;
pub use xyz::*;

#[macro_export]
macro_rules! colors {
    ($($name:ident => $storage:ident, $mul:ident, $size:expr), +) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name {
                data: [$storage; $size],
            }

            impl $name {
                pub fn new(data: [$storage; $size]) -> Self {
                    debug_assert!(data.iter().all(|f| !f.is_nan()));
                    Self { data }
                }

                pub fn new_const(data: $storage) -> Self {
                    Self::new([data; $size])
                }

                pub fn sqrt(&self) -> Self {
                    let mut data = self.data;
                    data.iter_mut().for_each(|f| *f = f.sqrt());

                    Self::new(data)
                }

                pub fn lerp(&self, other: &Self, t: f32) -> Self {
                    *self * (1.0 - t) + *other * t
                }

                /// Clamps the color values between min and max.
                pub fn clamp(&self, min: f32, max: f32) -> Self {
                    let mut data = self.data;
                    floats::fast_clamp_ar(&mut data, min, max);

                    Self::new(data)
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    let data = [$storage::default(); $size];
                    Self::new(data)
                }
            }

            impl Add for $name {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] += rhs.data[i];
                    }

                    Self::new(data)
                }
            }

            impl AddAssign for $name {
                fn add_assign(&mut self, rhs: Self) {
                    for i in 0..self.data.len() {
                        self.data[i] += rhs.data[i];
                    }
                }
            }

            impl Sub for $name {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] -= rhs.data[i];
                    }

                    Self::new(data)
                }
            }

            impl SubAssign for $name {
                fn sub_assign(&mut self, rhs: Self) {
                    for i in 0..self.data.len() {
                        self.data[i] -= rhs.data[i];
                    }
                }
            }

            impl Mul for $name {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] *= rhs.data[i];
                    }

                    Self::new(data)
                }
            }

            impl MulAssign for $name {
                fn mul_assign(&mut self, rhs: Self) {
                    for i in 0..self.data.len() {
                        self.data[i] *= rhs.data[i];
                    }
                }
            }

            impl Mul<$mul> for $name {
                type Output = Self;

                fn mul(self, rhs: $mul) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] *= rhs;
                    }

                    Self::new(data)
                }
            }

            impl MulAssign<$mul> for $name {
                fn mul_assign(&mut self, rhs: $mul) {
                    for i in 0..self.data.len() {
                        self.data[i] *=  rhs;
                    }
                }
            }

            impl Div for $name {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] /= rhs.data[i];
                    }

                    Self::new(data)
                }
            }

            impl DivAssign for $name {
                fn div_assign(&mut self, rhs: Self) {
                    for i in 0..self.data.len() {
                        self.data[i] /= rhs.data[i];
                    }
                }
            }

            impl Div<$mul> for $name {
                type Output = Self;

                fn div(self, rhs: $mul) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] /= rhs;
                    }

                    Self::new(data)
                }
            }

            impl DivAssign<$mul> for $name {
                fn div_assign(&mut self, rhs: $mul) {
                    for i in 0..self.data.len() {
                        self.data[i] /= rhs;
                    }
                }
            }

            impl Index<usize> for $name {
                type Output = $storage;

                fn index(&self, index: usize) -> &Self::Output {
                    &self.data[index]
                }
            }

            impl IndexMut<usize> for $name {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    &mut self.data[index]
                }
            }

            impl PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    self.data.iter().zip(other.data.iter()).all(|(d0, d1)| d0 == d1)
                }
            }

            impl Eq for $name {}

            impl Sum for $name {
                fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                    iter.fold($name::default(), |a, b| a + b)
                }
            }

            impl Into<$name> for f32 {
                fn into(self) -> $name {
                    $name::new_const(self)
                }
            }
        )+
    }
}

pub trait Color:
Add
+ AddAssign
+ Sub
+ SubAssign
+ Mul
+ MulAssign
+ Mul<f32>
+ MulAssign<f32>
+ Div
+ DivAssign
+ Div<f32>
+ DivAssign<f32>
+ PartialEq
+ Index<usize>
+ IndexMut<usize>
+ Debug
+ Into<Rgb<u8>>
+ Into<Rgb<u16>>
+ Sum
{
    /// Whether this color is black. Some computations can be omitted, if the color is black.
    fn is_black(&self) -> bool;

    /// Clamps the color values between min and max.
    fn clamp(&self, min: f32, max: f32) -> Self;

    /// Whether this color has NaN values.
    fn has_nans(&self) -> bool;

    fn sqrt(&self) -> Self;

    /// Converts this color to sRGB.
    fn to_rgb(&self) -> Srgb;

    /// Converts this color to XYZ.
    fn to_xyz(&self) -> Xyz;

    fn black() -> Self;

    fn white() -> Self;

    fn red() -> Self;

    fn green() -> Self;

    fn blue() -> Self;
}

/// Returns the XYZ to sRGB matrix
#[allow(clippy::excessive_precision)]
pub fn xyz_to_srgb_mat() -> Mat3 {
    // https://en.wikipedia.org/wiki/SRGB#The_forward_transformation_(CIE_XYZ_to_sRGB)
    Mat3::new(
        Vec3::new(3.240_97, -0.96924364, 0.05563008),
        Vec3::new(-1.53738318, 1.8759675, -0.20397696),
        Vec3::new(-0.49861076, 0.04155506, 1.05697151),
    )
}

/// Returns the sRGB to XYZ matrix
#[allow(clippy::excessive_precision)]
pub fn srgb_to_xyz_mat() -> Mat3 {
    // https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
    Mat3::new(
        Vec3::new(0.41239080, 0.21263901, 0.01933082),
        Vec3::new(0.35758434, 0.71516868, 0.07219232),
        Vec3::new(0.18048079, 0.07219232, 0.95053215),
    )
}

/// Converts sRGB to linear
#[allow(clippy::excessive_precision)]
pub fn srgb_to_linear(val: f32) -> f32 {
    assert!(val >= 0.0);
    assert!(val <= 1.0);
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.0404482362771082 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}

/// Converts sRGB to linear
pub fn srgbs_to_linear(val: Vec3) -> Vec3 {
    val.map(srgb_to_linear)
}

/// Converts linelar to sRGB
#[allow(clippy::excessive_precision)]
pub fn linear_to_srgb(val: f32) -> f32 {
    assert!(val >= 0.0);
    assert!(val <= 1.0);
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.00313066844250063 {
        val * 12.92
    } else {
        1.055 * val.powf(1.0 / 2.4) - 0.055
    }
}

/// Converts linelar to sRGB
pub fn linears_to_srgb(val: Vec3) -> Vec3 {
    val.map(linear_to_srgb)
}

