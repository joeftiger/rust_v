use crate::render::camera::Camera;
use crate::render::scene::Scene;
use image::{Rgb, RgbImage};
use ultraviolet::Vec3;
use crate::{Spectrum, floats};
use crate::color::Color;

pub trait Renderer: Send + Sync {
    fn is_done(&self) -> bool;

    fn reset(&mut self);

    fn render_all(&mut self);

    fn render_pass(&mut self);

    fn get_camera(&mut self) -> &mut Camera;

    fn get_image(&self) -> RgbImage;
}

#[allow(dead_code)]
pub mod debug {
    use crate::render::camera::Camera;
    use crate::render::renderer::Renderer;
    use crate::render::scene::Scene;
    use crate::Spectrum;
    use image::{Rgb, RgbImage};
    use ultraviolet::Vec3;

    pub struct NormalRenderer {
        scene: Scene,
        camera: Camera,
        image: RgbImage,
        progress: u32,
    }

    unsafe impl Send for NormalRenderer {}
    unsafe impl Sync for NormalRenderer {}

    impl NormalRenderer {
        pub fn new(scene: Scene, camera: Camera) -> Self {
            let image = RgbImage::new(camera.width, camera.height);

            Self {
                scene,
                camera,
                image,
                progress: 0,
            }
        }

        fn render(&self, x: u32, y: u32) -> Rgb<u8> {
            let ray = self.camera.primary_ray(x, y);

            let si = self.scene.intersect(&ray);

            if let Some(si) = si {
                let normal = (si.info.normal + Vec3::one()) / 2.0;
                let color = Spectrum::from(normal);

                color.into()
            } else {
                Rgb::from([0, 0, 0])
            }
        }

        fn inc_progress(&mut self) {
            self.progress += 1;
        }
    }

    impl Renderer for NormalRenderer {
        fn is_done(&self) -> bool {
            self.progress >= self.image.width() * self.image.height()
        }

        fn reset(&mut self) {
            self.progress = 0;
        }

        fn render_all(&mut self) {
            if !self.is_done() {
                for x in 0..self.image.width() {
                    for y in 0..self.image.height() {
                        let pixel = self.render(x, y);
                        self.image.put_pixel(x, y, pixel);
                    }
                }
            }
        }

        fn render_pass(&mut self) {
            if !self.is_done() {
                let x = self.progress % self.image.width();
                let y = self.progress / self.image.width();

                let pixel = self.render(x, y);

                self.image.put_pixel(x, y, pixel);
                self.inc_progress();
            }
        }

        fn get_camera(&mut self) -> &mut Camera {
            &mut self.camera
        }

        fn get_image(&self) -> RgbImage {
            self.image.clone()
        }
    }
}

pub struct RgbRenderer {
    scene: Scene,
    camera: Camera,
    image: RgbImage,
    progress: u32,
}

unsafe impl Send for RgbRenderer {}
unsafe impl Sync for RgbRenderer {}

impl RgbRenderer {
    pub fn new(scene: Scene, camera: Camera) -> Self {
        let image = RgbImage::new(camera.width, camera.height);

        Self {
            scene,
            camera,
            image,
            progress: 0,
        }
    }

    fn render(&self, x: u32, y: u32) -> Rgb<u8> {
        let ray = self.camera.primary_ray(x, y);

        let si = self.scene.intersect(&ray);

        if let Some(si) = si {
            let normal = si.info.normal;
            let view = -si.info.ray.direction;

            let obj = self.scene.get_obj(si.obj_id);
            let color: Spectrum = self.scene.lights
                .iter()
                .filter(|l| self.scene.is_occluded(&l.ray_to(si.info.point)))
                .map(|l| {
                    let dir = l.direction_from(si.info.point);
                    let cos = normal.dot(dir).max(0.0);

                    let intensity = l.intensity_at(si.info.point);

                    obj.bxdf.apply(view, dir) * (intensity * cos)
                })
                .sum();

            color.into()
        } else {
            Rgb::from([0, 0, 0])
        }
    }

    fn inc_progress(&mut self) {
        self.progress += 1;
    }
}

impl Renderer for RgbRenderer {
    fn is_done(&self) -> bool {
        self.progress >= self.image.width() * self.image.height()
    }

    fn reset(&mut self) {
        self.progress = 0;
    }

    fn render_all(&mut self) {
        if !self.is_done() {
            for x in 0..self.image.width() {
                for y in 0..self.image.height() {
                    let pixel = self.render(x, y);
                    self.image.put_pixel(x, y, pixel);
                }
            }
        }
    }

    fn render_pass(&mut self) {
        if !self.is_done() {
            let x = self.progress % self.image.width();
            let y = self.progress / self.image.width();

            let pixel = self.render(x, y);

            self.image.put_pixel(x, y, pixel);
            self.inc_progress();
        }
    }

    fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    fn get_image(&self) -> RgbImage {
        self.image.clone()
    }
}
