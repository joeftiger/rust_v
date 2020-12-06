// To allow debug checks
#![allow(clippy::float_cmp)]

#[macro_use]
extern crate bitflags;

use color::*;

pub mod configuration;
mod demos;
pub mod plain_scene;
pub mod render;

pub type Spectrum = Srgb;
