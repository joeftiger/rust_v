// To allow debug checks
#![allow(clippy::float_cmp)]

#[macro_use]
extern crate bitflags;

use color::*;

pub mod configuration;
mod demo_scenes;
pub mod plain_scene;
pub mod render;
pub mod shapes;
pub mod mc;
pub mod bxdf;
pub mod integrator;
pub mod sampler;

pub type Spectrum = Srgb;

pub const LIGHT_SAMPLES_1D: usize = 2;
pub const LIGHT_SAMPLES_3D: usize = LIGHT_SAMPLES_1D * LIGHT_SAMPLES_1D * LIGHT_SAMPLES_1D;
