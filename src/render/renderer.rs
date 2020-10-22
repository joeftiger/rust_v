use image::{Rgb, RgbImage};

use crate::acceleration_structure::AccelerationStructure;
use crate::acceleration_structure::no_acceleration::NoAcceleration;
use crate::geometry::CeilFloorExt;
use crate::render::{Camera, Scene};

pub trait Renderer: Send + Sync {
    fn render(&mut self) -> RgbImage;

    fn render_pass(&mut self) -> bool;

    fn reset(&mut self);

    fn get_image(&self) -> RgbImage;

    fn get_scene(&mut self) -> &mut Scene;

    fn get_camera(&mut self) -> &mut Camera;
}

pub struct DummyRgbRenderer {
    scene: Scene,
    image: RgbImage,
    progress: (u32, u32),
}

impl DummyRgbRenderer {
    pub fn new(scene: Scene) -> Self {
        let camera = &scene.camera;
        let image = RgbImage::new(camera.width, camera.height);
        let progress = (0, 0);

        Self {
            scene,
            image,
            progress,
        }
    }

    fn render_pixel(&self, xy: (u32, u32)) -> Rgb<u8> {
        let r = fastrand::u8(..255);
        let g = fastrand::u8(..255);
        let b = fastrand::u8(..255);
        Rgb::from([r, g, b])
    }

    fn reset_progress(&mut self) {
        self.progress = (0, 0);
    }

    fn inc_progress(&mut self) -> bool {
        let size = (self.image.width(), self.image.height());

        let mut progress = self.progress;
        progress.0 += 1;

        if progress.0 >= size.0 {
            progress.0 = 0;
            progress.1 += 1;
        }

        self.progress = progress;

        self.is_progress_done()
    }

    fn is_progress_done(&self) -> bool {
        self.progress.1 >= self.image.height() || self.progress.0 >= self.image.width()
    }
}

impl Renderer for DummyRgbRenderer {
    fn render(&mut self) -> RgbImage {
        self.reset_progress();

        loop {
            if !self.render_pass() {
                break;
            }
        }

        self.get_image()
    }

    fn render_pass(&mut self) -> bool {
        if !self.is_progress_done() {
            let (x, y) = self.progress;
            let pixel = self.render_pixel(self.progress);
            self.image.put_pixel(x, y, pixel);
            self.inc_progress()
        } else {
            true
        }
    }

    fn reset(&mut self) {
        self.image = RgbImage::new(self.image.width(), self.image.height());
        self.reset_progress();
    }

    fn get_image(&self) -> RgbImage {
        self.image.clone()
    }

    fn get_scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn get_camera(&mut self) -> &mut Camera {
        &mut self.scene.camera
    }
}

pub struct RgbRenderer<T> {
    scene: Scene,
    accelerator: T,
    image: RgbImage,
    progress: (u32, u32),
}

impl<T: AccelerationStructure> RgbRenderer<T> {
    pub fn new(scene: Scene, accelerator: T) -> Self {
        let camera = &scene.camera;
        let image = RgbImage::new(camera.width, camera.height);
        let progress = (0, 0);

        Self {
            scene,
            accelerator,
            image,
            progress,
        }
    }

    fn render_pixel(&self, xy: (u32, u32)) -> Rgb<u8> {
        let ray = self.scene.camera.primary_ray(xy.0, xy.1);

        let i = self.accelerator.accelerate(&ray, &self.scene);

        if let Some(intersection) = i.0 {
            if let Some(SRGB) = i.1 {
                let srgb = (SRGB.to_vec() * 255.0).floor();
                let rgb = [
                    srgb.x as u8,
                    srgb.y as u8,
                    srgb.z as u8
                ];

                println!("{:?}", intersection);
                println!("{:?}", SRGB);

                Rgb::from(rgb)
            } else {
                Rgb::from([255, 255, 255])
            }
        } else {
            Rgb::from([0, 0, 0])
        }
    }

    fn reset_progress(&mut self) {
        self.progress = (0, 0);
    }

    fn inc_progress(&mut self) -> bool {
        let size = (self.image.width(), self.image.height());

        let mut progress = self.progress;
        progress.0 += 1;

        if progress.0 >= size.0 {
            progress.0 = 0;
            progress.1 += 1;
        }

        self.progress = progress;

        self.is_progress_done()
    }

    fn is_progress_done(&self) -> bool {
        self.progress.1 >= self.image.height() || self.progress.0 >= self.image.width()
    }
}

impl<T: AccelerationStructure> Renderer for RgbRenderer<T> {
    fn render(&mut self) -> RgbImage {
        self.reset_progress();

        loop {
            if !self.render_pass() {
                break;
            }
        }

        self.get_image()
    }

    fn render_pass(&mut self) -> bool {
        if !self.is_progress_done() {
            let (x, y) = self.progress;
            let pixel = self.render_pixel(self.progress);
            self.image.put_pixel(x, y, pixel);
            self.inc_progress()
        } else {
            true
        }
    }

    fn reset(&mut self) {
        self.image = RgbImage::new(self.image.width(), self.image.height());
        self.reset_progress();
    }

    fn get_image(&self) -> RgbImage {
        self.image.clone()
    }

    fn get_scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn get_camera(&mut self) -> &mut Camera {
        &mut self.scene.camera
    }
}
