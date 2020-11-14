use ultraviolet::Vec2;

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

pub trait Sampler {
    fn get_1d(&mut self) -> f32;

    fn get_2d(&mut self) -> Vec2 {
        Vec2::new(self.get_1d(), self.get_1d())
    }

    fn get_sample(&mut self) -> Sample {
        Sample::new(self.get_1d(), self.get_2d())
    }
}

/// A simple Sampler only returning random numbers.
pub struct RandomSampler;

impl Sampler for RandomSampler {
    fn get_1d(&mut self) -> f32 {
        0.5
        // let rand = fastrand::f32();
        // debug_assert_ne!(rand, 1.0);
        // rand
    }
}
