#![allow(dead_code)]
#![allow(unused_variables)]

use crate::Spectrum;
use geometry::Geometry;
use std::sync::Arc;

pub struct Emitter {
    geometry: Arc<dyn Geometry>,
    pub emission: Spectrum,
}

impl Emitter {
    pub fn new(geometry: Arc<dyn Geometry>, emission: Spectrum) -> Self {
        Self { geometry, emission }
    }
}
