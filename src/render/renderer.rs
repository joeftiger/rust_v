use crate::render::camera::Camera;
use crate::render::scene::Scene;
use crate::{floats, Spectrum};
use image::{ImageBuffer, Rgb};

pub trait Renderer: Send + Sync {
    fn is_done(&self) -> bool;

    fn reset(&mut self);

    fn render_all(&mut self);

    fn render_pass(&mut self);

    fn get_camera(&mut self) -> &mut Camera;

    fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>>;

    fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>>;
}

fn convert_u16_to_u8(vec: Vec<u16>) -> Vec<u8> {
    vec.iter().map(|b16| (b16 / 2u16.pow(8)) as u8).collect()
}

#[allow(dead_code)]
pub mod debug {
    use crate::render::camera::Camera;
    use crate::render::renderer::{convert_u16_to_u8, Renderer};
    use crate::render::scene::Scene;
    use crate::Spectrum;
    use image::{ImageBuffer, Rgb};
    use ultraviolet::Vec3;

    pub struct NormalRenderer {
        scene: Scene,
        camera: Camera,
        image: ImageBuffer<Rgb<u16>, Vec<u16>>,
        progress: u32,
    }

    unsafe impl Send for NormalRenderer {}
    unsafe impl Sync for NormalRenderer {}

    impl NormalRenderer {
        pub fn new(scene: Scene, camera: Camera) -> Self {
            let image = ImageBuffer::new(camera.width, camera.height);

            Self {
                scene,
                camera,
                image,
                progress: 0,
            }
        }

        fn render(&self, x: u32, y: u32) -> Rgb<u16> {
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

        fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
            let data = convert_u16_to_u8(self.image.to_vec());

            ImageBuffer::from_vec(self.image.width(), self.image.height(), data)
                .expect("Could not convert u16 image to u8")
        }

        fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
            self.image.clone()
        }
    }
}

pub struct RgbRenderer {
    scene: Scene,
    camera: Camera,
    image: ImageBuffer<Rgb<u16>, Vec<u16>>,
    progress: u32,
}

unsafe impl Send for RgbRenderer {}
unsafe impl Sync for RgbRenderer {}

impl RgbRenderer {
    pub fn new(scene: Scene, camera: Camera) -> Self {
        let image = ImageBuffer::new(camera.width, camera.height);

        Self {
            scene,
            camera,
            image,
            progress: 0,
        }
    }

    fn render(&self, x: u32, y: u32) -> Rgb<u16> {
        let ray = self.camera.primary_ray(x, y);

        let si = self.scene.intersect(&ray);

        if let Some(si) = si {
            let point = si.info.point;
            let normal = si.info.normal;
            let view = -si.info.ray.direction;

            let obj = self.scene.get_obj(si.obj_id);

            let mut color = Spectrum::default();
            for light in &self.scene.lights {
                // exact vector
                let to_light = light.direction_from(point);

                // offset actual ray to avoid black pixels
                let mut ray = light.ray_to(point);
                ray.origin += normal * floats::BIG_EPSILON;

                // normal diffuse color
                // TODO: Subject to change
                color += obj.bxdf.apply(view, to_light);

                if !self.scene.is_occluded(&ray) {
                    let cos = normal.dot(to_light).max(0.0);
                    let intensity = light.intensity_at(point);

                    color += obj.bxdf.apply(view, to_light) * intensity * cos;
                }
            }

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

    fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let data = convert_u16_to_u8(self.image.to_vec());

        ImageBuffer::from_vec(self.image.width(), self.image.height(), data)
            .expect("Could not convert u16 image to u8")
    }

    fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
        self.image.clone()
    }
}
