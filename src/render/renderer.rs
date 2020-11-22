use std::sync::atomic::{AtomicU32, Ordering};
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

        Self {
            scene,
            camera,
            sampler,
            integrator,
            spectrum_statistics,
            image: Arc::new(RwLock::new(image)),
            progress: Arc::new(RwLock::new(0)),
            img_width,
            img_height,
        }
    }

    pub fn len_pixels(&self) -> u32 {
        self.img_width * self.img_height
    }

    pub fn get_progress(&self) -> u32 {
        *self.progress.read().unwrap()
    }

    pub fn is_done(&self) -> bool {
        self.get_progress() >= self.len_pixels()
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

    pub fn render_all(&mut self, passes: u32) {
        if self.is_done() {
            self.reset_progress()
        }

        for x in 0..self.img_width {
            for y in 0..self.img_height {
                for _ in 0..passes {
                    let pixel = self.render(x, y);
                    self.image.write().unwrap().put_pixel(x, y, pixel.into());
                }
            }
        }
    }

    pub fn render_all_with(&mut self, passes: u32, bar: Arc<Mutex<ProgressBar>>) {
        if self.is_done() {
            self.reset_progress()
        }

        let len_pixels = self.len_pixels();
        let percent = len_pixels / 100;

        let counter = AtomicU32::new(0);
        (0..len_pixels).into_par_iter().for_each(|curr_index| {
            let x = curr_index % self.img_width;
            let y = curr_index / self.img_height;

            let mut this = self.clone();

            for _ in 0..passes {
                let pixel = this.render(x, y);
                {
                    let mut img = this.image.write().expect("Image poisoned");
                    img.put_pixel(x, y, pixel.into());
                }
            }

            let prev = counter.fetch_add(1, Ordering::SeqCst);
            if prev % percent == 0 {
                // println!("{}", prev);
                bar.lock()
                    .expect("ProgressBar poisoned")
                    .inc_length(percent as u64)
            }
        });

        // while curr_index < len_pixels {
        //     let x = curr_index % self.img_width;
        //     let y = curr_index / self.img_height;
        //
        //     let mut this = self.clone();
        //
        //     pool.spawn(move ||
        //         for _ in 0..passes {
        //             let pixel = this.render(x, y);
        //             this.image.write().unwrap().put_pixel(x, y, pixel.into());
        //         }
        //     );
        //
        //     curr_index += 1;
        //
        //     if curr_index % percent == 0 {
        //         bar.inc_length(percent as u64)
        //     }
        // }
    }

    pub fn render_pass(&mut self) {
        if !self.is_done() {
            let progress = self.get_progress();
            let x = progress % self.img_width;
            let y = progress / self.img_height;

            let pixel = self.render(x, y);

            {
                let stats = &mut self.spectrum_statistics.lock().unwrap()[progress as usize];

                stats.spectrum += pixel;
                stats.num += 1;

                self.image
                    .write()
                    .unwrap()
                    .put_pixel(x, y, stats.average().into());
            }

            *self.progress.write().unwrap() += 1;
        }
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
