use image::{ImageBuffer, Rgb, RgbImage, Rgba};
use ultraviolet::Vec3;

use crate::geometry::ray::Ray;
use crate::render::objects::SceneObject;

pub mod material;
pub mod objects;
pub mod window;
pub mod renderer;

#[derive(Default)]
pub struct Size {
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
}

unsafe impl Send for Size {}
unsafe impl Sync for Size {}

impl Size {
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

unsafe impl Send for CameraInfo {}
unsafe impl Sync for CameraInfo {}

impl CameraInfo {
    pub fn new(position: Vec3, center: Vec3, up: Vec3, fovy: f32) -> Self {
        Self {
            position,
            center,
            up,
            fovy,
        }
    }
}

#[derive(Default)]
pub struct Camera {
    pub camera_info: CameraInfo,
    pub image_size: Size,

    // private fields
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
}

unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}

impl Camera {
    pub fn new(camera_info: CameraInfo, image_dimension: Size) -> Self {
        let mut camera = Self::default();
        camera.camera_info = camera_info;
        camera.image_size = image_dimension;
        camera.update();

        camera
    }

    /// Updates the camera in respect to its CameraInfo and ImageDimension
    pub fn update(&mut self) {
        let dir = self.camera_info.center - self.camera_info.position;
        let view = dir.normalized();
        let dist = dir.mag();

        let w = self.image_size.width as f32;
        let h = self.image_size.height as f32;

        let height = 2.0 * dist * f32::tan(0.5 * self.camera_info.fovy);
        let width = w * height / h;

        let x_dir = view.cross(self.camera_info.up).normalized() * width / height;
        let y_dir = x_dir.cross(view).normalized() * height / width;
        let lower_left = self.camera_info.center - 0.5 * (w * x_dir - h * y_dir);

        self.x_dir = x_dir;
        self.y_dir = y_dir;
        self.lower_left = lower_left;
    }

    pub fn primary_ray(&self, x: u32, y: u32) -> Ray {
        let origin = self.camera_info.position;
        let direction =
            self.lower_left + (x as f32) * self.x_dir + (y as f32) * self.y_dir - origin;

        Ray::new(origin, direction)
    }
}

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn SceneObject>>,
    pub camera: Camera,
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

impl Scene {
    pub fn new(objects: Vec<Box<dyn SceneObject>>, camera: Camera) -> Self {
        Self { objects, camera }
    }
}
