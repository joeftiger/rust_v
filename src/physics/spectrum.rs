use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Default)]
pub struct SpectrumEntry {
    pub wave_length_nm: f32,
    pub radiance: f32,
}

pub struct Spectrum<const SAMPLES: usize> {
    pub spectrum: [SpectrumEntry; SAMPLES],
}

impl SpectrumEntry {
    pub fn new(wave_length_nm: f32, radiance: f32) -> Self {
        Self {
            wave_length_nm,
            radiance,
        }
    }

    pub fn new_wave_length_nm(wave_length_nm: f32) -> Self {
        Self {
            wave_length_nm,
            radiance: f32::default(),
        }
    }
}

pub trait CoefficientSpectrum<const SAMPLES: usize>:
Add
+ AddAssign
+ Sub
+ SubAssign
+ Mul
+ MulAssign
+ Div
+ DivAssign
+ Mul<f32>
+ MulAssign<f32>
+ Div<f32>
+ DivAssign<f32>
+ Eq
+ Sized
{
    /// Creates and assigns the given value to the whole spectrum.
    fn new(value: f32) -> Self;

    /// Creates and assigns the values of the other spectrum.
    fn from(other: Self) -> Self;
}

// TODO: WIP
// impl CoefficientSpectrum<3> for Spectrum<3> {
//     fn new(value: f32) -> Self {
//         Self {
//             spectrum: [
//                 SpectrumEntry::new(667.5, value),
//                 SpectrumEntry::new(540.0, value),
//                 SpectrumEntry::new(470.0, value)
//             ]
//         }
//     }
//
//     fn from(other: Self) -> Self {
//         Self {
//             spectrum: [
//                 other.spectrum[0].clone(),
//                 other.spectrum[1].clone(),
//                 other.spectrum[2].clone(),
//             ]
//         }
//     }
// }
