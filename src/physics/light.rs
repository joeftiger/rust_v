use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeTuple;

use crate::Float;

type RGB = Spectrum<3>;

#[must_use]
fn new_data_from<const SAMPLES: usize, F: Iterator<Item=Float>>(src: F) -> [Float; SAMPLES] {
    let mut result = [0.0; SAMPLES];
    for (rref, val) in result.iter_mut().zip(src) {
        *rref = val;
    }

    result
}

pub struct Spectrum<const SAMPLES: usize> {
    data: [Float; SAMPLES],
}

impl<const SAMPLES: usize> Spectrum<SAMPLES> {
    pub fn new_empty() -> Self {
        let data = [0 as Float; SAMPLES];

        Self { data }
    }

    pub fn new(data: [Float; SAMPLES]) -> Self {
        Self { data }
    }

    /// Whether the whole spectrum is zero (<= epsilon)
    pub fn is_black(&self) -> bool {
        self.data.iter().all(|radiance| *radiance <= Float::EPSILON)
    }

    /// Whether the whole spectrum is constant (delta <= epsilon)
    pub fn is_const(&self) -> bool {
        assert!(!self.data.is_empty());

        let reference = self.data[0];
        self.data.iter().all(|radiance| Float::abs(radiance - reference) <= Float::EPSILON)
    }
}

impl<const SAMPLES: usize> Index<usize> for Spectrum<SAMPLES> {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const SAMPLES: usize> Add for Spectrum<SAMPLES> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data = new_data_from(self.data.iter().zip(&rhs.data).map(|(a, b)| a + b));

        Self { data }
    }
}

impl<const SAMPLES: usize> AddAssign for Spectrum<SAMPLES> {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.data.iter_mut().zip(&rhs.data) {
            *a += *b;
        }
    }
}

impl<const SAMPLES: usize> Sub for Spectrum<SAMPLES> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data = new_data_from(self.data.iter().zip(&rhs.data).map(|(a, b)| a - b));

        Self { data }
    }
}

impl<const SAMPLES: usize> SubAssign for Spectrum<SAMPLES> {
    fn sub_assign(&mut self, rhs: Self) {
        for (a, b) in self.data.iter_mut().zip(&rhs.data) {
            *a -= *b;
        }
    }
}

impl<const SAMPLES: usize> Mul for Spectrum<SAMPLES> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = new_data_from(self.data.iter().zip(&rhs.data).map(|(a, b)| a * b));

        Self { data }
    }
}

impl<const SAMPLES: usize> MulAssign for Spectrum<SAMPLES> {
    fn mul_assign(&mut self, rhs: Self) {
        for (a, b) in self.data.iter_mut().zip(&rhs.data) {
            *a *= *b;
        }
    }
}

impl<const SAMPLES: usize> Mul<Float> for Spectrum<SAMPLES> {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        let data = new_data_from(self.data.iter().map(|a| a * rhs));

        Self { data }
    }
}

impl<const SAMPLES: usize> MulAssign<Float> for Spectrum<SAMPLES> {
    fn mul_assign(&mut self, rhs: Float) {
        for a in self.data.iter_mut() {
            *a *= rhs;
        }
    }
}

impl<const SAMPLES: usize> Div for Spectrum<SAMPLES> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data = new_data_from(self.data.iter().zip(&rhs.data).map(|(a, b)| a / b));

        Self { data }
    }
}

impl<const SAMPLES: usize> DivAssign for Spectrum<SAMPLES> {
    fn div_assign(&mut self, rhs: Self) {
        for (a, b) in self.data.iter_mut().zip(&rhs.data) {
            *a /= *b;
        }
    }
}

impl<const SAMPLES: usize> Div<Float> for Spectrum<SAMPLES> {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        let data = new_data_from(self.data.iter().map(|a| a / rhs));

        Self { data }
    }
}

impl<const SAMPLES: usize> DivAssign<Float> for Spectrum<SAMPLES> {
    fn div_assign(&mut self, rhs: Float) {
        for a in self.data.iter_mut() {
            *a /= rhs;
        }
    }
}

impl<const SAMPLES: usize> PartialEq for Spectrum<SAMPLES> {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().zip(&other.data).all(|(a, b)| a == b)
    }
}

impl<const SAMPLES: usize> Serialize for Spectrum<SAMPLES> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(SAMPLES)?;

        for elem in self.data.iter() {
            seq.serialize_element(&elem)?;
        }

        seq.end()
    }
}

impl RGB {
    pub fn new_rgb(r: Float, g: Float, b: Float) -> Self {
        Self { data: [r, g, b] }
    }
}
