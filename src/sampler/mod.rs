use ultraviolet::{Vec2, Vec3};

/// A sample consisting of a 1D and 2D sample.
#[derive(Debug)]
pub struct Sample {
    pub one_d: f32,
    pub two_d: Vec2,
}

impl Sample {
    pub fn new(one_d: f32, two_d: Vec2) -> Self {
        Self { one_d, two_d }
    }
}

pub trait Sampler: Send + Sync {
    fn get_1d(&self) -> f32;

    #[inline]
    fn get_2d(&self) -> Vec2 {
        Vec2::new(self.get_1d(), self.get_1d())
    }

    #[inline]
    fn get_3d(&self) -> Vec3 {
        Vec3::new(self.get_1d(), self.get_1d(), self.get_1d())
    }

    #[inline]
    fn get_sample(&self) -> Sample {
        Sample::new(self.get_1d(), self.get_2d())
    }
}

/// A simple Sampler only returning random numbers.
pub struct RandomSampler;

impl Default for RandomSampler {
    fn default() -> Self {
        fastrand::seed(0);
        Self
    }
}

impl Sampler for RandomSampler {
    #[inline]
    fn get_1d(&self) -> f32 {
        let rand = fastrand::f32();
        debug_assert_ne!(rand, 1.0);
        rand
    }
}

/// A no-op Sampler only returning `0.5`.
pub struct NoopSampler;

impl Sampler for NoopSampler {
    fn get_1d(&self) -> f32 {
        0.5
    }
}
