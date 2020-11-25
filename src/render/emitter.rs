#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;
use geometry::Geometry;
use crate::Spectrum;

pub struct Emitter {
    geometry: Arc<dyn Geometry>,
    pub emission: Spectrum,
}

impl Emitter {
    pub fn new(geometry: Arc<dyn Geometry>, emission: Spectrum) -> Self {
        Self { geometry, emission }
    }
}
