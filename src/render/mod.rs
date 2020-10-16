use image::{ImageBuffer, Rgb, Rgba, RgbImage};

use crate::geometry::Intersectable;
use crate::render::objects::SceneObject;
use ultraviolet::Vec3;
use crate::geometry::ray::NormalRay;

pub mod material;
mod objects;
pub mod window;

#[derive(Default)]
pub struct ImageDimension {
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
}

impl ImageDimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Default)]
pub struct CameraInfo {
    /// Position of the camera in global space
    pub position: Vec3,
    /// The center of the scene the camera is looking at
    pub center: Vec3,
    /// The up-direction of the camera
    pub up: Vec3,
    /// field of view in y-direction (in radians)
    pub fovy: f32,
}

impl CameraInfo {
    pub fn new(position: Vec3, center: Vec3, up: Vec3, fovy: f32) -> Self {
        Self { position, center, up, fovy }
    }
}

#[derive(Default)]
pub struct Camera {
    pub camera_info: CameraInfo,
    pub image_dimension: ImageDimension,

    // private fields
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new(camera_info: CameraInfo, image_dimension: ImageDimension) -> Self {
        let mut camera = Self::default();
        camera.camera_info = camera_info;
        camera.image_dimension = image_dimension;
        camera.update();

        camera
    }

    /// Updates the camera in respect to its CameraInfo and ImageDimension
    pub fn update(&mut self) {
        let dir = self.camera_info.center - self.camera_info.position;
        let view = dir.normalized();
        let dist = dir.mag();

        let w = self.image_dimension.width as f32;
        let h = self.image_dimension.height as f32;

        let height = 2.0 * dist * f32::tan(0.5 * self.camera_info.fovy);
        let width = w * height / h;

        let x_dir = view.cross(self.camera_info.up).normalized() * width / height;
        let y_dir = x_dir.cross(view).normalized() * height / width;
        let lower_left = self.camera_info.center - 0.5 * (w * x_dir - h * y_dir);

        self.x_dir = x_dir;
        self.y_dir = y_dir;
        self.lower_left = lower_left;
    }

    pub fn primary_ray(&self, x: u32, y: u32) -> NormalRay {
        let origin = self.camera_info.position;
        let direction = self.lower_left + (x as f32) * self.x_dir + (y as f32) * self.y_dir - origin;

        NormalRay::new(origin, direction)
    }
}


pub enum ColourMode {
    RGB,
    Spectral,
}

pub enum MonteCarlo {
    Random,
    Importance,
    HeroWaveLength,
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn SceneObject>>,
    camera: Camera,
}

pub trait Renderer {
    fn render(&self) -> RgbImage;

    fn render_pass(&self);

    fn reset(&self);

    fn get_image(&self) -> RgbImage;

    fn get_scene(&mut self) -> &mut Scene;
    // fn get_camera(&self) -> None;
}

pub struct RgbRenderer {
    scene: Scene,
}

impl RgbRenderer {
    pub fn new(scene: Scene) -> Self {
        Self { scene }
    }
}

impl RgbRenderer {
    pub fn dummy_render() -> RgbImage {
        let width = 640;
        let height = 480;
        let mut image = RgbImage::new(width, height);

        for x in (width / 2 - 4)..(width / 2 + 4) {
            for y in (height / 2 - 16)..(height / 2 + 16) {
                image.put_pixel(x, y, Rgb([255, 0, 0]));
            }
        }
        for x in (width / 2 - 16)..(width / 2 + 16) {
            for y in (height / 2 - 4)..(height / 2 + 4) {
                image.put_pixel(x, y, Rgb([0, 0, 255]));
            }
        }

        image
    }
}

impl Renderer for RgbRenderer {
    fn render(&self) -> RgbImage {
        let dim = &self.scene.camera.image_dimension;
        RgbImage::new(dim.width, dim.height)
    }

    fn render_pass(&self) {
        unimplemented!()
    }

    fn reset(&self) {
        unimplemented!()
    }

    fn get_image(&self) -> RgbImage {
        unimplemented!()
    }

    fn get_scene(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
