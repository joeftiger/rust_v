use ultraviolet::Vec3;

pub struct SimpleMaterial {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
    pub mirror: f32,
    pub refractiveness: f32,
    pub transmissive: bool,
}

impl SimpleMaterial {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32, mirror: f32, refractiveness: f32, transmissive: bool) -> Self {
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
