use crate::geometry::ray::Ray;
use ultraviolet::Vec3;

/// A camera consists of
/// - position: camera center
/// - center: center of the scene, which the camera is looking at
/// - up: vector specifying the up direction
/// - fovy: opening angle (field of view) in y-direction
/// - width: width of the image in pixels
/// - height: height of the image in pixels
pub struct Camera {
    pub position: Vec3,
    pub center: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub fovy: f32,
    pub width: u32,
    pub height: u32,

    // private fields
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, center: Vec3, up: Vec3, fovy: f32, width: u32, height: u32) -> Self {
        // compute orientation and distance of eye to scene center
        let forward = (center - position).normalized();
        let right = forward.cross(up).normalized();
        let up = right.cross(forward).normalized();
        let distance = (center - position).mag();

        // compute width & height of the image plane
        // based on the opening angle of the camera (fovy) and the distance
        // of the eye to the near plane (dist)
        let w = width as f32;
        let h = height as f32;
        let image_height = 2.0 * distance * f32::tan(0.5 * fovy * std::f32::consts::PI / 180.0);
        let image_width = w * image_height / h;

        let x_dir = right * image_width / w;
        let y_dir = -up * image_height / h;
        let lower_left = center - 0.5 * w * x_dir - 0.5 * h * y_dir;

        Self {
            position,
            center,
            forward,
            up,
            right,
            fovy,
            width,
            height,
            x_dir,
            y_dir,
            lower_left,
        }
    }

    pub fn primary_ray(&self, x: u32, y: u32) -> Ray {
        let origin = self.position;
        let direction =
            self.lower_left + (x as f32) * self.x_dir + (y as f32) * self.y_dir - origin;

        Ray::new(origin, direction, f32::INFINITY)
    }

    pub fn reset(&mut self) {
        *self = Self::new(
            self.position,
            self.center,
            self.up,
            self.fovy,
            self.width,
            self.height,
        );
    }
}
