use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::Geometry;
use crate::render::camera::Camera;
use crate::render::scene::Scene;
use crate::render::scene_objects::SceneObject;
use image::{Rgb, RgbImage};

pub trait Renderer {
    fn is_done(&self) -> bool;

    fn reset(&mut self);

    fn render_all(&mut self);

    fn render_pass(&mut self);

    fn get_camera(&mut self) -> &mut Camera;

    fn get_image(&self) -> RgbImage;
}

#[allow(dead_code)]
pub mod debug {
    use crate::color::Srgb;
    use crate::geometry::intersection::Intersection;
    use crate::geometry::ray::Ray;
    use crate::geometry::Geometry;
    use crate::render::camera::Camera;
    use crate::render::renderer::Renderer;
    use crate::render::scene::Scene;
    use crate::render::scene_objects::SceneObject;
    use image::{Rgb, RgbImage};

    pub struct NormalRenderer<T> {
        scene: Scene<T>,
        camera: Camera,
        image: RgbImage,
        progress: u32,
    }

    impl<T: Geometry<Ray, Intersection>> NormalRenderer<SceneObject<T>> {
        pub fn new(scene: Scene<SceneObject<T>>, camera: Camera) -> Self {
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
                let normal = si.intersection.normal.abs();
                let color = si.color * Srgb::from(normal);

                color.into()
            } else {
                Rgb::from([0, 0, 0])
            }
        }

        fn inc_progress(&mut self) {
            self.progress += 1;
        }
    }

    impl<T: Geometry<Ray, Intersection>> Renderer for NormalRenderer<SceneObject<T>> {
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

pub struct RgbRenderer<T> {
    scene: Scene<T>,
    camera: Camera,
    image: RgbImage,
    progress: u32,
}

impl<T: Geometry<Ray, Intersection>> RgbRenderer<SceneObject<T>> {
    pub fn new(scene: Scene<SceneObject<T>>, camera: Camera) -> Self {
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
            si.color.into()
        } else {
            Rgb::from([0, 0, 0])
        }
    }

    fn inc_progress(&mut self) {
        self.progress += 1;
    }
}

impl<T: Geometry<Ray, Intersection>> Renderer for RgbRenderer<SceneObject<T>> {
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
