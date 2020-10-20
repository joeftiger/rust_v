// use std::cmp::Ordering;
// use std::ops::Index;
//
// #[derive(Clone, Default)]
// pub struct LightWave {
//     pub wavelength_nm: f32,
//     pub radiance: f32,
// }
//
// impl LightWave {
//     pub fn new(wavelength_nm: f32, radiance: f32) -> Self {
//         Self { wavelength_nm, radiance }
//     }
//
//     pub fn new_wave(wavelength_nm: f32) -> Self {
//         Self { wavelength_nm, radiance: f32::default() }
//     }
//
//     pub fn new_radiance(radiance: f32) -> Self {
//         Self { wavelength_nm: f32::default(), radiance }
//     }
// }
//
// impl PartialEq for LightWave {
//     fn eq(&self, other: &Self) -> bool {
//         self.wavelength_nm == other.wavelength_nm
//     }
// }
//
// impl Eq for LightWave {}
//
// impl PartialOrd for LightWave {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.wavelength_nm.partial_cmp(&other.wavelength_nm)
//     }
// }
//
// impl Ord for LightWave {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.wavelength_nm.total_cmp(&other.wavelength_nm)
//     }
// }
//
// pub struct CoefficientSpectrum<const SAMPLES: usize> {
//     data: [LightWave; SAMPLES],
// }
//
// impl<const SAMPLES: usize> CoefficientSpectrum<SAMPLES> {
//     pub fn new(data: [LightWave; SAMPLES]) -> Self {
//         Self { data }
//     }
//
//     /// Returns the closest next lower index of the given wavelength (if not empty).
//     /// O(log2(n))
//     fn index_of_next_lower_wavelength(&self, wavelength_nm: f32) -> Option<usize> {
//         let mut left = 0;
//         let mut right = self.data.len() - 1;
//
//         if self.data.len() == 1 {
//             return Some(0);
//         }
//
//         while left <= right {
//             let middle = f32::floor((left + right) as f32 / 2.0) as usize;
//
//             if self[middle].wavelength_nm > wavelength_nm {
//                 right = middle - 1;
//             } else if middle > 0 && self[middle - 1].wavelength_nm < wavelength_nm {
//                 left = middle + 1;
//             } else {
//                 Some(middle);
//             }
//         }
//
//         None
//     }
//
//     /// Returns the closest next upper index of the given wavelength (if not empty).
//     /// O(log2(n))
//     pub fn index_of_next_upper_wavelength(&self, wavelength_nm: f32) -> Option<usize> {
//         let last_index = self.data.len() - 1;
//         let mut left = 0;
//         let mut right = last_index;
//
//         if self.data.len() == 1 {
//             return Some(0);
//         }
//
//         while left <= right {
//             let middle = f32::floor((left + right) as f32 / 2.0) as usize;
//
//             if self[middle].wavelength_nm < wavelength_nm {
//                 left = middle + 1;
//             } else if middle < last_index && self[middle + 1].wavelength_nm > wavelength_nm {
//                 right = middle - 1;
//             } else {
//                 Some(middle);
//             }
//         }
//
//         None
//     }
//
//     /// Uses linear interpolation for given wave length.
//     pub fn lerp_radiance(&self, wavelength_nm: f32) -> Option<LightWave> {
//         if let Some(min_index) = self
//             .index_of_next_lower_wavelength(wavelength_nm)
//         {
//             if min_index + 1 >= self.data.len() {
//                 return Some(LightWave::new(wavelength_nm, self[min_index].radiance));
//             }
//
//             let s1 = &self[min_index];
//             let s2 = &self[min_index + 1];
//
//             let interpolation = s1.radiance + (wavelength_nm - s1.wavelength_nm) * (s2.radiance - s1.radiance) / (s2.wavelength_nm - s1.wavelength_nm);
//
//             return Some(LightWave::new(wavelength_nm, interpolation));
//         }
//
//         None
//     }
//
//     /// Uses polynomial interpolation in lagrange form for given wave length.
//     pub fn perp_radiance(&self, wavelength_nm: f32) -> Option<LightWave> {
//         if self.data.len() == 0 {
//             return None;
//         }
//
//         let mut radiance = 0.0;
//         for i in 0..self.data.len() {
//             let mut product = 1.0;
//
//             for j in 0..self.data.len() {
//                 if i != j {
//                     product *= (wavelength_nm - self[j].wavelength_nm) / (self[i].wavelength_nm / self[j].wavelength_nm);
//                 }
//             }
//
//             radiance += product * self[i].radiance;
//         }
//
//         Some(LightWave::new(wavelength_nm, radiance))
//     }
//
//     /// Whether the whole spectrum is zero (<= epsilon)
//     pub fn is_black(&self) -> bool {
//         self.data.iter().all(|lw| lw.radiance <= f32::EPSILON)
//     }
//
//     /// Whether the whole spectrum is constant (delta <= epsilon)
//     pub fn is_const(&self) -> bool {
//         assert!(!self.data.is_empty());
//
//         let reference = self.data[0].radiance;
//
//         self.data.iter().all(|lw| f32::abs(lw.radiance - reference) <= f32::EPSILON)
//     }
// }
//
// impl<const SAMPLES: usize> Index<usize> for CoefficientSpectrum<SAMPLES> {
//     type Output = LightWave;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }
