use crate::geometry::{Boxable, Intersectable};
use crate::geometry::ray::Ray;
use image::{RgbImage, DynamicImage, GenericImage, Rgba, GenericImageView, ImageBuffer, Rgb};

pub mod material;

pub enum ColourMode {
    RGB,
    Spectral,
}

pub enum MonteCarlo {
    Random,
    Importance,
    HeroWaveLength,
}

pub trait SceneObject: Boxable + Intersectable<Box<dyn Ray>> {}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn SceneObject>>,
}

pub trait Renderer {
    fn render(&self, scene: &Scene) -> RgbImage;
}

pub struct RgbRenderer {
    width: u32,
    height: u32,
}

impl RgbRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl RgbRenderer {
    pub fn dummy_render() -> RgbImage {
        let width = 640;
        let height = 480;
        let mut image = RgbImage::new(width, height);

        for x in (width / 2 - 4)..(width / 2 + 4) {
            for y in (height / 2 - 16)..(height / 2 + 16){
                image.put_pixel(x, y, Rgb([255, 0, 0]));
            }
        }
        for x in (width / 2 - 16)..(width / 2 + 16) {
            for y in (height / 2 - 4)..(height / 2 + 4){
                image.put_pixel(x, y, Rgb([0, 0, 255]));
            }
        }

        image
    }
}

impl Renderer for RgbRenderer {
    fn render(&self, scene: &Scene) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);

        for x in (self.width / 2 - 4)..(self.width / 2 + 4) {
            for y in (self.height / 2 - 16)..(self.height / 2 + 16){
                image.put_pixel(x, y, Rgb([255, 0, 0]));
            }
        }
        for x in (self.width / 2 - 16)..(self.width / 2 + 16) {
            for y in (self.height / 2 - 4)..(self.height / 2 + 4){
                image.put_pixel(x, y, Rgb([255, 0, 0]));
            }
        }

        image
    }
}
