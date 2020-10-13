use std::cmp::Ordering;
use std::ops::Index;

use crate::util::sorted_vec::SortedVec;

#[derive(Clone, Default)]
pub struct LightWave {
    pub wave_length_nm: f32,
    pub radiance: f32,
}

impl LightWave {
    pub fn new(wave_length_nm: f32, radiance: f32) -> Self {
        Self { wave_length_nm, radiance }
    }

    pub fn new_wave(wave_length_nm: f32) -> Self {
        Self { wave_length_nm, radiance: f32::default() }
    }

    pub fn new_radiance(radiance: f32) -> Self {
        Self { wave_length_nm: f32::default(), radiance }
    }
}

#[derive(Default)]
pub struct Spectrum {
    spectrum: SortedVec<LightWave>,
}

impl PartialEq for LightWave {
    fn eq(&self, other: &Self) -> bool {
        self.wave_length_nm == other.wave_length_nm
    }
}

impl Eq for LightWave {}

impl PartialOrd for LightWave {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.wave_length_nm.partial_cmp(&other.wave_length_nm)
    }
}

impl Ord for LightWave {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wave_length_nm.total_cmp(&other.wave_length_nm)
    }
}

impl Spectrum {
    pub fn new(spectrum: SortedVec<LightWave>) -> Self {
        Self { spectrum }
    }

    /// Uses linear interpolation for given wave length.
    pub fn lerp_radiance(&self, wave_length_nm: f32) -> Option<LightWave> {
        if let Some(min_index) = self
            .spectrum
            .index_of_next_lower(LightWave::new_wave(wave_length_nm))
        {
            if min_index + 1 >= self.spectrum.len() {
                return Some(LightWave::new(wave_length_nm, self.spectrum[min_index].radiance));
            }

            let s1 = &self.spectrum[min_index];
            let s2 = &self.spectrum[min_index + 1];

            let interpolation = s1.radiance + (wave_length_nm - s1.wave_length_nm) * (s2.radiance - s1.radiance) / (s2.wave_length_nm - s1.wave_length_nm);

            return Some(LightWave::new(wave_length_nm, interpolation));
        }

        None
    }

    /// Uses polynomial interpolation in lagrange form for given wave length.
    pub fn perp_radiance(&self, wave_length_nm: f32) -> Option<LightWave> {
        if self.spectrum.is_empty() {
            return None;
        }

        let mut radiance = 0.0;
        for i in 0..self.spectrum.len() {
            let mut product = 1.0;

            for j in 0..self.spectrum.len() {
                if i != j {
                    product *= (wave_length_nm - self[j].wave_length_nm) / (self[i].wave_length_nm / self[j].wave_length_nm);
                }
            }

            radiance += product * self[i].radiance;
        }

        Some(LightWave::new(wave_length_nm, radiance))
    }
}

impl Index<usize> for Spectrum {
    type Output = LightWave;

    fn index(&self, index: usize) -> &Self::Output {
        &self.spectrum[index]
    }
}
