use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use ultraviolet::Vec3;

use crate::physics::{Color, linear_to_srgb, srgb_to_linear, xyz_to_srgb_mat, linears_to_srgb};
use crate::physics::rgb::SRGB;
use crate::util::floats::{approx_equal, approx_zero, fast_clamp};

// TODO: IS this representation wrong? I find Yxy in the internet as well
#[derive(Clone, Default)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        debug_assert!(!x.is_nan());
        debug_assert!(!y.is_nan());
        debug_assert!(!z.is_nan());
        Self { x, y, z }
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Color for XYZ {
    fn is_black(&self) -> bool {
        approx_zero(self.x) && approx_zero(self.y) && approx_zero(self.z)
    }

    fn clamp(&self, min: f32, max: f32) -> Self {
        let x = fast_clamp(self.x, min, max);
        let y = fast_clamp(self.y, min, max);
        let z = fast_clamp(self.z, min, max);

        Self { x, y, z }
    }

    fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    fn to_rgb(&self) -> SRGB {
        SRGB::from(linears_to_srgb(xyz_to_srgb_mat() * self.to_vec()))
    }

    fn to_xyz(&self) -> XYZ {
        self.clone()
    }
}

impl Add for XYZ {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Self::new(x, y, z)
    }
}

impl AddAssign for XYZ {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for XYZ {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        Self::new(x, y, z)
    }
}

impl SubAssign for XYZ {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for XYZ {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;

        Self::new(x, y, z)
    }
}

impl MulAssign for XYZ {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for XYZ {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        debug_assert!(!rhs.is_nan());
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;

        Self::new(x, y, z)
    }
}

impl MulAssign<f32> for XYZ {
    fn mul_assign(&mut self, rhs: f32) {
        debug_assert!(!rhs.is_nan());
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for XYZ {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;

        Self { x, y, z }
    }
}

impl DivAssign for XYZ {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<f32> for XYZ {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(!rhs.is_nan());
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;

        Self::new(x, y, z)
    }
}

impl DivAssign<f32> for XYZ {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for XYZ {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index [{}] out of range for XYZ", index)
        }
    }
}

impl IndexMut<usize> for XYZ {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index [{}] out of range for XYZ", index)
        }
    }
}

impl PartialEq for XYZ {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.x, other.x) && approx_equal(self.y, other.y) && approx_equal(self.z, other.z)
    }
}

impl From<(f32, f32, f32)> for XYZ {
    fn from(xyz: (f32, f32, f32)) -> Self {
        let x = xyz.0;
        let y = xyz.1;
        let z = xyz.2;
        Self::new(x, y, z)
    }
}

impl From<[f32; 3]> for XYZ {
    fn from(xyz: [f32; 3]) -> Self {
        let x = xyz[0];
        let y = xyz[1];
        let z = xyz[2];
        Self::new(x, y, z)
    }
}

impl From<Vec3> for XYZ {
    fn from(xyz: Vec3) -> Self {
        let x = xyz.x;
        let y = xyz.y;
        let z = xyz.z;
        Self::new(x, y, z)
    }
}

impl From<SRGB> for XYZ {
    fn from(srgb: SRGB) -> Self {
        srgb.to_xyz()
    }
}
