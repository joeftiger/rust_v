use crate::render::camera::Camera;
use image::{RgbImage, Rgb};
use crate::render::scene::Scene;
use crate::geometry::Geometry;
use crate::geometry::ray::Ray;
use crate::geometry::intersection::Intersection;
use crate::render::scene_objects::SceneObject;

pub trait Renderer {
    fn is_done(&self) -> bool;

    fn reset(&mut self);

    fn render_pass(&mut self);

    fn get_camera(&mut self) -> &mut Camera;

    fn get_image(&self) -> RgbImage;
}

pub struct RgbRenderer<T> {
    scene: Scene<T>,
    camera: Camera,
    image: RgbImage,
    progress: (u32, u32),
}

impl<T: Geometry<Ray, Intersection>> RgbRenderer<SceneObject<T>> {
    pub fn new(scene: Scene<SceneObject<T>>, camera: Camera) -> Self {
        let image = RgbImage::new(camera.width, camera.height);
        let progress = (0, 0);

        Self { scene, camera, image, progress }
    }

    fn render(&self, x: u32, y: u32) -> Rgb<u8> {
        let ray = self.camera.primary_ray(x, y);

        let si = self.scene.intersect(&ray);

        if let Some(si) = si {
            si.color.into()
        } else {
            Rgb::from([0, 0, 0])
        }
    }

    fn inc_progress(&mut self) {
        let size = (self.image.width(), self.image.height());

        let mut progress = self.progress;
        progress.0 += 1;

        if progress.0 >= size.0 {
            progress.0 = 0;
            progress.1 += 1;
        }

        self.progress = progress;
    }
}

impl<T: Geometry<Ray, Intersection>> Renderer for RgbRenderer<SceneObject<T>> {
    fn is_done(&self) -> bool {
        self.progress.1 >= self.image.height() || self.progress.0 >= self.image.width()
    }

    fn reset(&mut self) {
        self.progress = (0, 0);
    }

    fn render_pass(&mut self) {
        if !self.is_done() {
            let (x, y) = self.progress;
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
