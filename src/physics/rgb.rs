use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use ultraviolet::Vec3;

use crate::physics::{Color, srgb_to_xyz_mat, srgbs_to_linear};
use crate::physics::xyz::XYZ;
use crate::util::floats::{approx_equal, approx_zero, fast_clamp};

#[derive(Clone, Default)]
pub struct SRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl SRGB {
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

impl Color for SRGB {
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

    fn to_rgb(&self) -> SRGB {
        self.clone()
    }

    fn to_xyz(&self) -> XYZ {
        XYZ::from(srgb_to_xyz_mat() * srgbs_to_linear(self.to_vec()))
    }
}

impl Add for SRGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r + rhs.r;
        let g = self.g + rhs.g;
        let b = self.b + rhs.b;

        Self::new(r, g, b)
    }
}

impl AddAssign for SRGB {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for SRGB {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.r - rhs.r;
        let g = self.g - rhs.g;
        let b = self.b - rhs.b;

        Self::new(r, g, b)
    }
}

impl SubAssign for SRGB {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul for SRGB {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.r * rhs.r;
        let g = self.g * rhs.g;
        let b = self.b * rhs.b;

        Self::new(r, g, b)
    }
}

impl MulAssign for SRGB {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f32> for SRGB {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        debug_assert!(!rhs.is_nan());
        let r = self.r * rhs;
        let g = self.g * rhs;
        let b = self.b * rhs;

        Self::new(r, g, b)
    }
}

impl MulAssign<f32> for SRGB {
    fn mul_assign(&mut self, rhs: f32) {
        debug_assert!(!rhs.is_nan());
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div for SRGB {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let r = self.r / rhs.r;
        let g = self.g / rhs.g;
        let b = self.b / rhs.b;

        Self::new(r, g, b)
    }
}

impl DivAssign for SRGB {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl Div<f32> for SRGB {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(!rhs.is_nan());
        let r = self.r / rhs;
        let g = self.g / rhs;
        let b = self.b / rhs;

        Self::new(r, g, b)
    }
}

impl DivAssign<f32> for SRGB {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Index<usize> for SRGB {
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

impl IndexMut<usize> for SRGB {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Index [{}] out of range for RGB", index)
        }
    }
}

impl PartialEq for SRGB {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.r, other.r) && approx_equal(self.g, other.g) && approx_equal(self.b, other.b)
    }
}

impl From<(f32, f32, f32)> for SRGB {
    fn from(srgb: (f32, f32, f32)) -> Self {
        let x = srgb.0;
        let y = srgb.1;
        let z = srgb.2;
        Self::new(x, y, z)
    }
}

impl From<[f32; 3]> for SRGB {
    fn from(rgb: [f32; 3]) -> Self {
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        Self::new(r, g, b)
    }
}

impl From<Vec3> for SRGB {
    fn from(rgb: Vec3) -> Self {
        let r = rgb.x;
        let g = rgb.y;
        let b = rgb.z;
        Self::new(r, g, b)
    }
}
