use ultraviolet::Vec2;

pub trait Sampler {
    fn get_1d(&mut self) -> f32;

    fn get_1ds(&mut self, samples: &mut [f32]) {
        samples
            .iter_mut()
            .for_each(|sample| *sample = self.get_1d());
    }

    fn get_2d(&mut self) -> Vec2;

    fn get_2ds(&mut self, samples: &mut [Vec2]) {
        samples
            .iter_mut()
            .for_each(|sample| *sample = self.get_2d());
    }
}

/// A simple Sampler only returning random numbers.
pub struct RandomSampler;

impl Sampler for RandomSampler {
    fn get_1d(&mut self) -> f32 {
        let rand = fastrand::f32();
        debug_assert_ne!(rand, 1.0);
        rand
    }

    fn get_2d(&mut self) -> Vec2 {
        Vec2::new(self.get_1d(), self.get_1d())
    }
}
