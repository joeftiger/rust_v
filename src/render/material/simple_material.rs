use ultraviolet::Vec3;
use crate::Float;

pub struct SimpleMaterial {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: Float,
    pub mirror: Float,
    pub refractiveness: Float,
    pub transmissive: bool,
}

impl SimpleMaterial {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: Float, mirror: Float, refractiveness: Float, transmissive: bool) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            mirror,
            refractiveness,
            transmissive
        }
    }
}
