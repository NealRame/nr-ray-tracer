use glam::{
    DVec2,
    DVec3,
};

use rand::{distr::uniform::SampleRange, Rng};

pub trait FromRng {
    fn from_rng<T: Rng>(rng: &mut T) -> Self;
    fn from_rng_ranged<T: Rng, R: SampleRange<f64> + Clone>(rng: &mut T, range: R) -> Self;
}

impl FromRng for DVec3 {
    fn from_rng<T: Rng>(
        rng: &mut T,
    ) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }

    fn from_rng_ranged<T: Rng, R: SampleRange<f64> + Clone>(
        rng: &mut T,
        range: R,
    ) -> Self {
        Self {
            x: rng.random_range(range.clone()),
            y: rng.random_range(range.clone()),
            z: rng.random_range(range.clone()),
        }
    }
}

impl FromRng for DVec2 {
    fn from_rng<T: Rng>(
        rng: &mut T,
    ) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
        }
    }

    fn from_rng_ranged<T: Rng, R: SampleRange<f64> + Clone>(
        rng: &mut T,
        range: R,
    ) -> Self {
        Self {
            x: rng.random_range(range.clone()),
            y: rng.random_range(range.clone()),
        }
    }
}
