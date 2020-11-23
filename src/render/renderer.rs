use std::sync::{Arc, RwLock, Mutex};

use color::Color;
use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::render::camera::Camera;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::Scene;
use crate::Spectrum;
use std::ops::AddAssign;
use util::range_block::RangeBlock;

pub const BLOCK_SIZE: u32 = 64;

// fn convert_u16_to_u8(vec: Vec<u16>) -> Vec<u8> {
//     vec.iter().map(|b16| (b16 / 2u16.pow(8)) as u8).collect()
// }

fn convert_to_u8(vec: &[Mutex<SpectrumStatistic>]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vec.len() * 3);

    vec.iter()
        .for_each(|px| {
            let color = px.lock().expect("Image is poisoned").average().to_rgb();
            out.push((color[0] * 2u32.pow(8) as f32) as u8);
            out.push((color[1] * 2u32.pow(8) as f32) as u8);
            out.push((color[2] * 2u32.pow(8) as f32) as u8);
        });

    out
}

fn convert_to_u16(vec: &[Mutex<SpectrumStatistic>]) -> Vec<u16> {
    let mut out = Vec::with_capacity(vec.len() * 3);

    vec.iter()
        .for_each(|px| {
            let color = px.lock().expect("Image is poisoned").average().to_rgb();
            out.push((color[0] * 2u32.pow(16) as f32) as u16);
            out.push((color[1] * 2u32.pow(16) as f32) as u16);
            out.push((color[2] * 2u32.pow(16) as f32) as u16);
        });

    out
}

// fn convert_to_u8(vec: Arc<Vec<RwLock<Spectrum>>>) -> Vec<Rgb<u8>> {
//     vec.iter()
//         .map(|px| px.read().expect("Image is poisoned").to_rgb().into()).collect()
// }
//
// fn convert_to_u16(vec: Arc<Vec<RwLock<Spectrum>>>) -> Vec<Rgb<u16>> {
//     vec.iter()
//         .map(|px| px.read().expect("Image is poisoned").to_rgb().into()).collect()
// }

#[derive(Default)]
struct SpectrumStatistic {
    spectrum: Spectrum,
    num: usize,
}

impl SpectrumStatistic {
    pub fn average(&self) -> Spectrum {
        if self.num == 0 {
            Spectrum::black()
        } else {
            self.spectrum / self.num as f32
        }
    }

    pub fn reset(&mut self) {
        self.num = 0;
        self.spectrum = Spectrum::black();
    }
}

#[allow(clippy::rc_buffer)]
#[derive(Clone)]
pub struct Renderer {
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    sampler: Arc<dyn Sampler>,
    integrator: Arc<dyn Integrator>,
    spectrum_statistics: Arc<Vec<Mutex<SpectrumStatistic>>>,
    progress: Arc<RwLock<u32>>,
    render_blocks: Arc<RangeBlock>,
    img_width: u32,
    img_height: u32,
}

impl Renderer {
    pub fn new(
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        sampler: Arc<dyn Sampler>,
        integrator: Arc<dyn Integrator>,
    ) -> Self {
        let (img_width, img_height) = (camera.width, camera.height);
        let capacity = (img_width * img_height) as usize;

        let spectrum_statistics = Arc::new(
            (0..capacity)
                .map(|_| Mutex::new(SpectrumStatistic::default()))
                .collect(),
        );

        let render_blocks = RangeBlock::new(img_width, img_height, 64);

        Self {
            scene,
            camera,
            sampler,
            integrator,
            spectrum_statistics,
            progress: Arc::new(RwLock::new(0)),
            render_blocks: Arc::new(render_blocks),
            img_width,
            img_height,
        }
    }

    pub fn num_blocks(&self) -> usize {
        self.render_blocks.num_blocks()
    }

    pub fn num_pixels(&self) -> u32 {
        self.img_width * self.img_height
    }

    pub fn get_progress(&self) -> u32 {
        *self.progress.read().unwrap()
    }

    pub fn is_done(&self) -> bool {
        self.get_progress() >= self.num_blocks() as u32
    }

    fn render(&mut self, x: u32, y: u32) -> Spectrum {
        let sample = self.sampler.get_2d();
        let ray = self.camera.primary_ray(x, y, &sample);

        self.integrator
            .integrate(&self.scene, &ray, self.sampler.clone())
    }

    pub fn reset_progress(&mut self) {
        *self.progress.write().unwrap() = 0;
    }

    pub fn reset_image(&mut self) {
        self.spectrum_statistics.iter()
            .for_each(|px| px.lock().expect("Spectrum is poisoned").reset());
    }

    pub fn render_all(&mut self, passes: u32, bar: Arc<ProgressBar>) {
        if self.is_done() {
            self.reset_progress()
        }

        let num_blocks = self.num_blocks();

        (0..num_blocks)
            .for_each(|block| {
                let mut this = self.clone();
                let block = &this.render_blocks[block];

                block.prod().into_iter().for_each(|(x, y)| {
                    let x = x as u32;
                    let y = y as u32;

                    let index = (x * this.img_width + y) as usize;
                    for _ in 0..passes {
                        let pixel = this.render(x, y);
                        {
                            let stats = &mut this.spectrum_statistics[index].lock().expect("Spectrum Statistics poisoned");

                            stats.num += 1;
                            stats.spectrum += pixel;
                        };
                    }
                });
                bar.inc_length(1)
            });
    }

    pub fn render_all_par(&mut self, passes: u32, bar: Arc<ProgressBar>) {
        if self.is_done() {
            self.reset_progress()
        }

        let num_blocks = self.num_blocks();

        (0..num_blocks)
            .into_par_iter()
            .for_each(|block| {
                let mut this = self.clone();
                let block = &this.render_blocks[block];

                block.prod().into_iter().for_each(|(x, y)| {
                    let x = x as u32;
                    let y = y as u32;

                    let index = (x * this.img_width + y) as usize;
                    for _ in 0..passes {
                        let pixel = this.render(x, y);
                        {
                            let stats = &mut this.spectrum_statistics[index].lock().expect("Spectrum Statistics poisoned");

                            stats.num += 1;
                            stats.spectrum += pixel;
                        };
                    }
                });
                bar.inc_length(1)
            });
    }

    pub fn render_pass(&mut self) {
        let index = {
            let mut lock = self.progress.write().expect("Progress poisoned");
            let progress = *lock;
            lock.add_assign(1);
            progress
        } as usize;

        self.render_blocks[index]
            .prod()
            .into_par_iter()
            .for_each(|(x, y)| {
                let mut this = self.clone();
                let x = x as u32;
                let y = y as u32;

                let index = (x * this.img_width + y) as usize;

                let pixel = this.render(x, y);
                {
                    let stats = &mut this.spectrum_statistics[index].lock().expect("Spectrum Statistics poisoned");

                    stats.num += 1;
                    stats.spectrum += pixel;
                };
            });
    }

    pub fn get_camera(&mut self) -> Arc<Camera> {
        self.camera.clone()
    }

    pub fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let data = convert_to_u8(self.spectrum_statistics.as_slice());

        ImageBuffer::from_vec(self.img_width, self.img_height, data).unwrap()
    }

    pub fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
        let data = convert_to_u16(self.spectrum_statistics.as_slice());

        ImageBuffer::from_vec(self.img_width, self.img_height, data).unwrap()
    }
}
