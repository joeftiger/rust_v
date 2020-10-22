use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use image::Rgb;
use ultraviolet::Vec3;

use crate::color::{Color, srgb_to_xyz_mat, srgbs_to_linear};
use crate::floats::{approx_equal, approx_zero, fast_clamp};
use crate::color::xyz::Xyz;

#[derive(Clone, Debug, Default)]
pub struct Srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Srgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        debug_assert!(!r.is_nan());
        debug_assert!(!g.is_nan());
        debug_assert!(!b.is_nan());
        Self { r, g, b }.clamp(0.0, 1.0)
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}

impl Color for Srgb {
    fn is_black(&self) -> bool {
        approx_zero(self.r) && approx_zero(self.g) && approx_zero(self.b)
    }

    fn clamp(&self, min: f32, max: f32) -> Self {
        let r = fast_clamp(self.r, min, max);
        let g = fast_clamp(self.g, min, max);
        let b = fast_clamp(self.b, min, max);

        Self { r, g, b }
    }

    fn has_nans(&self) -> bool {
        self.r.is_nan() || self.g.is_nan() || self.b.is_nan()
    }

    fn to_rgb(&self) -> Srgb {
        self.clone()
    }

    fn to_xyz(&self) -> Xyz {
        Xyz::from(srgb_to_xyz_mat() * srgbs_to_linear(self.to_vec()))
    }
}

impl Add for Srgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r + rhs.r;
        let g = self.g + rhs.g;
        let b = self.b + rhs.b;

        Self::new(r, g, b)
    }
}

impl AddAssign for Srgb {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Srgb {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.r - rhs.r;
        let g = self.g - rhs.g;
        let b = self.b - rhs.b;

        Self::new(r, g, b)
    }
}

impl SubAssign for Srgb {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul for Srgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.r * rhs.r;
        let g = self.g * rhs.g;
        let b = self.b * rhs.b;

        Self::new(r, g, b)
    }
}

impl MulAssign for Srgb {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f32> for Srgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        debug_assert!(!rhs.is_nan());
        let r = self.r * rhs;
        let g = self.g * rhs;
        let b = self.b * rhs;

        Self::new(r, g, b)
    }
}

impl MulAssign<f32> for Srgb {
    fn mul_assign(&mut self, rhs: f32) {
        debug_assert!(!rhs.is_nan());
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div for Srgb {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let r = self.r / rhs.r;
        let g = self.g / rhs.g;
        let b = self.b / rhs.b;

        Self::new(r, g, b)
    }
}

impl DivAssign for Srgb {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl Div<f32> for Srgb {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(!rhs.is_nan());
        let r = self.r / rhs;
        let g = self.g / rhs;
        let b = self.b / rhs;

        Self::new(r, g, b)
    }
}

impl DivAssign<f32> for Srgb {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Index<usize> for Srgb {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index [{}] out of range for RGB", index)
        }
    }
}

impl IndexMut<usize> for Srgb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Index [{}] out of range for RGB", index)
        }
    }
}

impl PartialEq for Srgb {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.r, other.r) && approx_equal(self.g, other.g) && approx_equal(self.b, other.b)
    }
}

impl From<(f32, f32, f32)> for Srgb {
    fn from(srgb: (f32, f32, f32)) -> Self {
        let x = srgb.0;
        let y = srgb.1;
        let z = srgb.2;
        Self::new(x, y, z)
    }
}

impl From<[f32; 3]> for Srgb {
    fn from(rgb: [f32; 3]) -> Self {
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        Self::new(r, g, b)
    }
}

impl From<Vec3> for Srgb {
    fn from(rgb: Vec3) -> Self {
        let r = rgb.x;
        let g = rgb.y;
        let b = rgb.z;
        Self::new(r, g, b)
    }
}

impl Into<Rgb<u8>> for Srgb {
    fn into(self) -> Rgb<u8> {
        let rgb = self.to_vec() * 255.0;

        Rgb::from([
            rgb.x as u8,
            rgb.y as u8,
            rgb.z as u8
        ])
    }
}
