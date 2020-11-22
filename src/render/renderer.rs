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
use util::range_block::RangeBlock;

pub const BLOCK_SIZE: u32 = 64;

fn convert_u16_to_u8(vec: Vec<u16>) -> Vec<u8> {
    vec.iter().map(|b16| (b16 / 2u16.pow(8)) as u8).collect()
}

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
}

#[derive(Clone)]
pub struct Renderer {
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    sampler: Arc<Mutex<dyn Sampler>>,
    integrator: Arc<dyn Integrator>,
    spectrum_statistics: Arc<Mutex<Vec<SpectrumStatistic>>>,
    image: Arc<RwLock<ImageBuffer<Rgb<u16>, Vec<u16>>>>,
    progress: Arc<RwLock<u32>>,
    render_blocks: Arc<RangeBlock>,
    img_width: u32,
    img_height: u32,
}

impl Renderer {
    pub fn new(
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        sampler: Arc<Mutex<dyn Sampler>>,
        integrator: Arc<dyn Integrator>,
    ) -> Self {
        let (img_width, img_height) = (camera.width, camera.height);
        let image = ImageBuffer::new(camera.width, camera.height);

        let capacity = (image.width() * image.height()) as usize;
        let spectrum_statistics = Arc::new(Mutex::new(
            (0..capacity)
                .map(|_| SpectrumStatistic::default())
                .collect(),
        ));

        let render_blocks = RangeBlock::new(img_width, img_height, 64);

        Self {
            scene,
            camera,
            sampler,
            integrator,
            spectrum_statistics,
            image: Arc::new(RwLock::new(image)),
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
        let sample = {
            let mut sampler = self.sampler.lock().expect("Sampler poisoned");
            sampler.get_2d()
        };
        let ray = self.camera.primary_ray(x, y, &sample);

        self.integrator
            .integrate(&self.scene, &ray, self.sampler.clone())
    }

    pub fn reset_progress(&mut self) {
        *self.progress.write().unwrap() = 0;
    }

    pub fn reset_image(&mut self) {
        self.image
            .write()
            .unwrap()
            .iter_mut()
            .for_each(|pixel| *pixel = 0);
    }

    pub fn render_all(&mut self, passes: u32, bar: Arc<Mutex<ProgressBar>>) {
        if self.is_done() {
            self.reset_progress()
        }

        let num_blocks = self.num_blocks();

        (0..num_blocks).for_each(|block| {
            let mut this = self.clone();
            let block = &this.render_blocks[block];

            block.prod().into_iter().for_each(|(x, y)| {
                let x = x as u32;
                let y = y as u32;

                for _ in 0..passes {
                    let pixel = this.render(x, y);
                    {
                        let mut img = this.image.write().expect("Image poisoned");
                        img.put_pixel(x, y, pixel.into());
                    }
                }
            });
            bar.lock().expect("ProgressBar poisoned").inc_length(1)
        });
    }

    pub fn render_all_par(&mut self, passes: u32, bar: Arc<Mutex<ProgressBar>>) {
        if self.is_done() {
            self.reset_progress()
        }

        let num_blocks = self.num_blocks();

        (0..num_blocks).into_par_iter().for_each(|block| {
            let mut this = self.clone();
            let block = &this.render_blocks[block];

            block.prod().into_iter().for_each(|(x, y)| {
                let x = x as u32;
                let y = y as u32;

                for _ in 0..passes {
                    let pixel = this.render(x, y);
                    {
                        let mut img = this.image.write().expect("Image poisoned");
                        img.put_pixel(x, y, pixel.into());
                    }
                }
            });
            bar.lock().expect("ProgressBar poisoned").inc_length(1)
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
            .into_par_iter()
            .for_each(|(x, y)| {
                let mut this = self.clone();
                let x = x as u32;
                let y = y as u32;

                let index = x * self.img_width + y;

                let pixel = this.render(x, y);
                {
                    let stats = &mut this.spectrum_statistics.lock().unwrap()[index as usize];

                    stats.spectrum += pixel;
                    stats.num += 1;

                    this.image.write().expect("Image poisoned").put_pixel(
                        x,
                        y,
                        stats.average().into(),
                    );
                }
            });
    }

    pub fn get_camera(&mut self) -> Arc<Camera> {
        self.camera.clone()
    }

    pub fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let data = convert_u16_to_u8(self.image.read().unwrap().to_vec());

        ImageBuffer::from_vec(self.img_width, self.img_height, data)
            .expect("Could not convert u16 image to u8")
    }

    pub fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
        self.image.read().unwrap().clone()
    }
}
