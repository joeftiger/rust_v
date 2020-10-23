use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::floats;
use image::Rgb;
use std::fmt::Debug;
use ultraviolet::{Mat3, Vec3};

pub mod cie;

macro_rules! colors {
    ($($name:ident => $storage:ident, $size:expr), +) => {
        $(
            #[derive(Clone, Debug)]
            pub struct $name {
                data: [$storage; $size],
            }

            impl $name {
                pub fn new(data: [$storage; $size]) -> Self {
                    Self { data }
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

            impl Mul<f32> for $name {
                type Output = Self;

                fn mul(self, rhs: f32) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] = (data[i] as f32 * rhs) as $storage;
                    }

                    Self::new(data)
                }
            }

            impl MulAssign<f32> for $name {
                fn mul_assign(&mut self, rhs: f32) {
                    for i in 0..self.data.len() {
                        self.data[i] = (self.data[i] as f32 * rhs) as $storage;
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

            impl Div<f32> for $name {
                type Output = Self;

                fn div(self, rhs: f32) -> Self::Output {
                    let mut data = self.data;
                    for i in 0..data.len() {
                        data[i] = (data[i] as f32 / rhs) as $storage;
                    }

                    Self::new(data)
                }
            }

            impl DivAssign<f32> for $name {
                fn div_assign(&mut self, rhs: f32) {
                    for i in 0..self.data.len() {
                        self.data[i] = (self.data[i] as f32 / rhs) as $storage;
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
        )+
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
colors!(
    Srgb => f32, 3,
    Xyz => f32, 3
);

/// Returns the XYZ to sRGB matrix
pub fn xyz_to_srgb_mat() -> Mat3 {
    // https://en.wikipedia.org/wiki/SRGB#The_forward_transformation_(CIE_XYZ_to_sRGB)
    Mat3::new(
        Vec3::new(3.24096994, -0.96924364, 0.05563008),
        Vec3::new(-1.53738318, 1.8759675, -0.20397696),
        Vec3::new(-0.49861076, 0.04155506, 1.05697151),
    )
}

/// Returns the sRGB to XYZ matrix
pub fn srgb_to_xyz_mat() -> Mat3 {
    // https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
    Mat3::new(
        Vec3::new(0.41239080, 0.21263901, 0.01933082),
        Vec3::new(0.35758434, 0.71516868, 0.07219232),
        Vec3::new(0.18048079, 0.07219232, 0.95053215),
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
{
    /// Whether this color is black. Some computations can be omitted, if the color is black.
    fn is_black(&self) -> bool;

    /// Clamps the color values between min and max.
    fn clamp(&self, min: f32, max: f32) -> Self;

    /// Whether this color has NaN values.
    fn has_nans(&self) -> bool;

    /// Converts this color to sRGB.
    fn to_rgb(&self) -> Srgb;

    /// Converts this color to XYZ.
    fn to_xyz(&self) -> Xyz;
}

impl Srgb {
    pub fn to_vec(&self) -> Vec3 {
        Vec3::from(self.data)
    }
}

impl Color for Srgb {
    fn is_black(&self) -> bool {
        self.data.iter().all(|value| floats::approx_zero(*value))
    }

    fn clamp(&self, min: f32, max: f32) -> Self {
        let mut data = self.data;
        floats::fast_clamp_ar(&mut data, min, max);

        Self::new(data)
    }

    fn has_nans(&self) -> bool {
        self.data.iter().all(|value| !value.is_nan())
    }

    fn to_rgb(&self) -> Srgb {
        self.clone()
    }

    fn to_xyz(&self) -> Xyz {
        Xyz::from(srgb_to_xyz_mat() * srgbs_to_linear(self.to_vec()))
    }
}

impl Into<Rgb<u8>> for Srgb {
    fn into(self) -> Rgb<u8> {
        let mut data = [0; 3];
        data.iter_mut()
            .zip(self.data.iter())
            .for_each(|(d0, d1)| *d0 = (d1 * 255.0) as u8);

        Rgb::from(data)
    }
}

impl From<Vec3> for Srgb {
    fn from(vec: Vec3) -> Self {
        Self::new([vec.x, vec.y, vec.z])
    }
}

impl Xyz {
    pub fn to_vec(&self) -> Vec3 {
        Vec3::from(self.data)
    }
}

impl Color for Xyz {
    fn is_black(&self) -> bool {
        self.data.iter().all(|value| floats::approx_zero(*value))
    }

    fn clamp(&self, min: f32, max: f32) -> Self {
        let mut data = self.data;
        floats::fast_clamp_ar(&mut data, min, max);

        Self::new(data)
    }

    fn has_nans(&self) -> bool {
        self.data.iter().all(|value| !value.is_nan())
    }

    fn to_rgb(&self) -> Srgb {
        Srgb::from(linears_to_srgb(xyz_to_srgb_mat() * self.to_vec()))
    }

    fn to_xyz(&self) -> Xyz {
        self.clone()
    }
}

impl Into<Rgb<u8>> for Xyz {
    fn into(self) -> Rgb<u8> {
        self.to_rgb().into()
    }
}

impl From<Vec3> for Xyz {
    fn from(vec: Vec3) -> Self {
        Self::new([vec.x, vec.y, vec.z])
    }
}
