use crate::render::camera::Camera;
use crate::render::scene::Scene;
use image::{ImageBuffer, Rgb};
use crate::render::sampler::Sampler;
use crate::render::integrator::Integrator;
use std::ops::DerefMut;
use crate::render::bxdf::BxDFType;

fn convert_u16_to_u8(vec: Vec<u16>) -> Vec<u8> {
    vec.iter().map(|b16| (b16 / 2u16.pow(8)) as u8).collect()
}

pub struct Renderer {
    scene: Scene,
    camera: Camera,
    sampler: Box<dyn Sampler>,
    integrator: Box<dyn Integrator>,
    image: ImageBuffer<Rgb<u16>, Vec<u16>>,
    progress: u32,
}

unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

impl Renderer {
    pub fn new(scene: Scene, camera: Camera,
               sampler: Box<dyn Sampler>,
               integrator: Box<dyn Integrator>,) -> Self {
        let image = ImageBuffer::new(camera.width, camera.height);

        Self {
            scene,
            camera,
            sampler,
            integrator,
            image,
            progress: 0,
        }
    }

    fn render(&mut self, x: u32, y: u32) -> Rgb<u16> {
        let ray = self.camera.primary_ray(x, y);

        let si = self.scene.intersect(&ray);

        if let Some(si) = si {
            let mut color = self.integrator.illumination(&self.scene, &si);
            let bsdf = &si.obj.bsdf;

            if bsdf.is_type(BxDFType::SPECULAR | BxDFType::REFLECTION) {
                color += self.integrator.specular_reflection(&self.scene, &si, self.sampler.deref_mut());
            }

            if bsdf.is_type(BxDFType::SPECULAR | BxDFType::TRANSMISSION) {
                color += self.integrator.specular_transmission(&self.scene, &si);
            }

            color.into()
        } else {
            Rgb::from([0, 0, 0])
        }
    }

    fn inc_progress(&mut self) {
        self.progress += 1;
    }

    pub fn is_done(&self) -> bool {
        self.progress >= self.image.width() * self.image.height()
    }

    pub fn reset(&mut self) {
        self.progress = 0;
    }

    pub fn render_all(&mut self) {
        if !self.is_done() {
            for x in 0..self.image.width() {
                for y in 0..self.image.height() {
                    let pixel = self.render(x, y);
                    self.image.put_pixel(x, y, pixel);
                }
            }
        }
    }

    pub fn render_pass(&mut self) {
        if !self.is_done() {
            let x = self.progress % self.image.width();
            let y = self.progress / self.image.width();

            let pixel = self.render(x, y);

            self.image.put_pixel(x, y, pixel);
            self.inc_progress();
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
