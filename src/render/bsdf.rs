use ultraviolet::Vec3;
use crate::render::scene::{Scene, SceneIntersection};
use crate::color::Srgb;
use crate::render::light::Light;

pub trait BSDF: Send + Sync {
    fn apply(
        &self,
        scene: &Scene,
        info: SceneIntersection,
    ) -> Srgb;
}

#[derive(Default)]
pub struct Phong {
    ambient: Srgb,
    diffuse: Srgb,
    specular: Srgb,
    shininess: f32,
}

impl Phong {
    pub fn new(ambient: Srgb, diffuse: Srgb, specular: Srgb, shininess: f32) -> Self {
        debug_assert!(!shininess.is_nan());
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    fn diffuse(&self, light: &Light, point: Vec3, normal: Vec3) -> Srgb {
        let factor = light.direction_from(point).dot(normal) * light.intensity_at(point);
        self.diffuse * factor
    }

    fn specular(&self, light: &Light, point: Vec3, view: Vec3) -> Srgb {
        // FIXME: This is negative at times, lading to factor=NaN
        let dot = light.direction_to(point).dot(view);

        let factor = dot.powf(self.shininess);
        debug_assert!(!factor.is_nan());
        self.specular * factor
    }
}

impl BSDF for Phong {
    fn apply(&self, scene: &Scene, intersection: SceneIntersection) -> Srgb {
        let partial = scene.lights
            .iter()
            .map(|light| {
                let diffuse = self.diffuse(light, intersection.info.point, intersection.info.normal);
                let specular = self.specular(light, intersection.info.point, intersection.info.ray.direction);

                diffuse + specular
            })
            .sum();

        let phong = self.ambient + partial;
        println!("{:?}", phong);

        phong
    }
}
