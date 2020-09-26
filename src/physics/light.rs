use std::cmp::Ordering;

use crate::util::sorted_vec::SortedVec;

#[derive(Clone, Default)]
pub struct Wave {
    wave_length_nm: f32,
}

#[derive(Clone, Default)]
struct SpectrumEntry {
    x: f32,
    y: f32,
}

pub struct Spectrum {
    spectrum: SortedVec<SpectrumEntry>,
}

impl Wave {
    pub fn new(wave_length_nm: f32) -> Self {
        Self {
            wave_length_nm
        }
    }
}

impl SpectrumEntry {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn new_x(x: f32) -> Self {
        Self {
            x,
            y: f32::default()
        }
    }
}

impl PartialEq for SpectrumEntry {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for SpectrumEntry {}

impl PartialOrd for SpectrumEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for SpectrumEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.total_cmp(&other.x)
    }
}

impl Spectrum {
    pub fn new(spectrum: SortedVec<SpectrumEntry>) -> Self {
        Self {
            spectrum
        }
    }

    /// Uses linear interpolation for given wave length.
    pub fn evaluate_linear(&self, wave_length: f32) -> Option<Wave> {
        if let Some(min_index) = self.spectrum.index_of_next_lower(SpectrumEntry::new_x(wave_length)) {
            if min_index + 1 >= self.spectrum.len() {
                return Some(Wave::new(self.spectrum[min_index].y));
            }

            let s1 = &self.spectrum[min_index];
            let s2 = &self.spectrum[min_index + 1];

            let interpolation = s1.y + (wave_length - s1.x) * (s2.y - s1.y) / (s2.x - s1.x);

            return Some(Wave::new(interpolation));
        }

        None
    }
}
