use crate::color::Color;
use crate::render::camera::Camera;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::Scene;
use crate::Spectrum;
use image::{ImageBuffer, Rgb};
use std::ops::DerefMut;

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

pub struct Renderer {
    scene: Scene,
    camera: Camera,
    sampler: Box<dyn Sampler>,
    integrator: Box<dyn Integrator>,
    spectrum_statistics: Vec<SpectrumStatistic>,
    image: ImageBuffer<Rgb<u16>, Vec<u16>>,
    progress: u32,
}

impl Renderer {
    pub fn new(
        scene: Scene,
        camera: Camera,
        sampler: Box<dyn Sampler>,
        integrator: Box<dyn Integrator>,
    ) -> Self {
        let image = ImageBuffer::new(camera.width, camera.height);

        let capacity = (image.width() * image.height()) as usize;
        let spectrum_statistics = (0..capacity)
            .map(|_| SpectrumStatistic::default())
            .collect();

        Self {
            scene,
            camera,
            sampler,
            integrator,
            spectrum_statistics,
            image,
            progress: 0,
        }
    }

    fn render(&mut self, x: u32, y: u32) -> Spectrum {
        let ray = self.camera.primary_ray(x, y);

        let si = self.scene.intersect(&ray);

        if let Some(si) = si {
            let sampler = self.sampler.deref_mut();

            self.integrator.integrate(&self.scene, &si, sampler)
        } else {
            Spectrum::black()
        }
    }

    pub fn is_done(&self) -> bool {
        self.progress >= self.image.width() * self.image.height()
    }

    pub fn reset_progress(&mut self) {
        self.progress = 0;
    }

    pub fn reset_image(&mut self) {
        self.image.iter_mut().for_each(|pixel| *pixel = 0);
    }

    pub fn render_all(&mut self) {
        if !self.is_done() {
            for x in 0..self.image.width() {
                for y in 0..self.image.height() {
                    let pixel = self.render(x, y);
                    self.image.put_pixel(x, y, pixel.into());
                }
            }
        }
    }

    pub fn render_pass(&mut self) {
        if !self.is_done() {
            let x = self.progress % self.image.width();
            let y = self.progress / self.image.width();

            let pixel = self.render(x, y);

            let index = (x * self.image.width() + y) as usize;
            let mut stats = &mut self.spectrum_statistics[index];
            stats.spectrum += pixel;
            stats.num += 1;

            self.image.put_pixel(x, y, stats.average().into());

            self.progress += 1;
        }
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let data = convert_u16_to_u8(self.image.to_vec());

        ImageBuffer::from_vec(self.image.width(), self.image.height(), data)
            .expect("Could not convert u16 image to u8")
    }

    pub fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
        self.image.clone()
    }
}
