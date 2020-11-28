use std::sync::{Arc, Mutex, RwLock};

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
use util::range_block::{Block, RangeBlock};

#[derive(Default)]
struct SpectrumStatistic {
    x: u32,
    y: u32,
    spectrum: Spectrum,
    samples: usize,
}

impl SpectrumStatistic {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            spectrum: Spectrum::black(),
            samples: 0,
        }
    }
    pub fn average(&self) -> Spectrum {
        if self.samples == 0 {
            self.spectrum
        } else {
            self.spectrum / self.samples as f32
        }
    }

    pub fn reset(&mut self) {
        self.samples = 0;
        self.spectrum = Spectrum::black();
    }
}

struct RenderBlock {
    stats: Vec<SpectrumStatistic>,
}

impl RenderBlock {
    pub fn reset(&mut self) {
        self.stats.iter_mut().for_each(|s| s.reset());
    }
}

impl From<&Block> for RenderBlock {
    fn from(block: &Block) -> Self {
        let stats = block
            .prod()
            .iter()
            .map(|(x, y)| SpectrumStatistic::new(*x as u32, *y as u32))
            .collect();
        Self { stats }
    }
}

#[derive(Clone)]
pub struct Renderer {
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    sampler: Arc<dyn Sampler>,
    integrator: Arc<dyn Integrator>,
    render_blocks: Arc<Vec<Mutex<RenderBlock>>>,
    progress: Arc<RwLock<u32>>,
    img_width: u32,
    img_height: u32,
}

impl Renderer {
    pub fn new(
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        sampler: Arc<dyn Sampler>,
        integrator: Arc<dyn Integrator>,
        block_size: u32,
    ) -> Self {
        let (img_width, img_height) = (camera.width, camera.height);

        let range_block = RangeBlock::new(img_width, img_height, block_size);
        let render_blocks = range_block
            .blocks
            .iter()
            .map(|block| Mutex::new(RenderBlock::from(block)))
            .collect();

        Self {
            scene,
            camera,
            sampler,
            integrator,
            progress: Arc::new(RwLock::new(0)),
            render_blocks: Arc::new(render_blocks),
            img_width,
            img_height,
        }
    }

    pub fn num_blocks(&self) -> usize {
        self.render_blocks.len()
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

    fn render(&self, x: u32, y: u32) -> Spectrum {
        let sample = self.sampler.get_2d();
        let ray = self.camera.primary_ray(x, y, &sample);

        self.integrator
            .integrate(&self.scene, &ray, self.sampler.clone())
    }

    pub fn reset_progress(&mut self) {
        *self.progress.write().unwrap() = 0;
    }

    pub fn reset_image(&mut self) {
        self.render_blocks
            .iter()
            .for_each(|b| b.lock().expect("Block is poisoned").reset());
    }

    pub fn render_all(&mut self, passes: u32, bar: &ProgressBar) {
        if self.is_done() {
            self.reset_progress()
        }

        self.render_blocks.iter().for_each(|block| {
            let this = self.clone();
            let mut lock = block.lock().expect("Block is poisoned");

            lock.stats.iter_mut().for_each(|stats| {
                for _ in 0..passes {
                    let pixel = this.render(stats.x, stats.y);
                    stats.spectrum += pixel;
                    stats.samples += 1;
                }
            });
            bar.inc(1)
        });
    }

    pub fn render_all_par(&mut self, passes: u32, bar: &ProgressBar) {
        if self.is_done() {
            self.reset_progress()
        }

        (0..self.num_blocks()).into_par_iter().for_each(|index| {
            let this = self.clone();

            let mut lock = this.render_blocks[index].lock().expect("Block is poisoned");
            lock.stats.iter_mut().for_each(|stats| {
                for _ in 0..passes {
                    let pixel = this.render(stats.x, stats.y);
                    stats.spectrum += pixel;
                    stats.samples += 1;
                }
            });
            bar.inc(1)
        });
    }

    pub fn render_pass(&mut self) {
        let num_cpus = num_cpus::get();
        let index = {
            let mut lock = self.progress.write().expect("Progress poisoned");
            let progress = *lock as usize;
            lock.add_assign(num_cpus as u32);
            progress..(progress + num_cpus).min(self.num_blocks())
        };

        // self.render_blocks[index].iter().for_each(|block| {
        self.render_blocks[index].par_iter().for_each(|block| {
            let this = self.clone();
            let mut block_lock = block.lock().expect("Block is poisoned");

            block_lock.stats.iter_mut().for_each(|px_stats| {
                let pixel = this.render(px_stats.x, px_stats.y);

                px_stats.spectrum += pixel;
                px_stats.samples += 1;
            });
        });
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Arc::new(camera);
    }

    pub fn get_camera(&mut self) -> Arc<Camera> {
        self.camera.clone()
    }

    pub fn get_image_u8(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut buffer = ImageBuffer::new(self.img_width, self.img_height);
        self.render_blocks.iter().for_each(|block| {
            let lock = block.lock().expect("Block is poisoned");

            lock.stats
                .iter()
                .for_each(|stat| buffer.put_pixel(stat.x, stat.y, stat.average().into()));
        });

        buffer
    }

    pub fn get_image_u16(&mut self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
        let mut buffer = ImageBuffer::new(self.img_width, self.img_height);
        self.render_blocks.iter().for_each(|block| {
            let lock = block.lock().expect("Block is poisoned");

            lock.stats
                .iter()
                .for_each(|stat| buffer.put_pixel(stat.x, stat.y, stat.average().into()));
        });

        buffer
    }
}
