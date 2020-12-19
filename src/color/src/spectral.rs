use crate::*;

pub const LAMBDA_START: f32 = 400.0;
pub const LAMBDA_END: f32 = 700.0;
pub const SPECTRAL_SAMPLES: usize = 60;

colors!(
    Spectral => f32, f32, SPECTRAL_SAMPLES
);

fn is_sorted(lambda: &[f32]) -> bool {
    let mut iter = lambda.iter();
    let mut prev = iter.next().unwrap();

    for next in iter {
        if prev > next {
            return false;
        }
        prev = next;
    }

    true
}

impl Spectral {
    pub fn from_sampled(lambda: &[f32], v: &[f32], n: usize) -> Self {
        assert_eq!(lambda.len(), n);
        assert_eq!(v.len(), n);

        if !is_sorted(lambda) {
            //     let permutation = permutation::sort_by(lambda, |a, b| floats::fast_cmp(*a, *b));
            //     let lambda_sorted = permutation.apply_slice(lambda).as_slice();
            //     let v_sorted = permutation.apply_slice(v).as_slice();
        }

        unimplemented!()
    }
}
