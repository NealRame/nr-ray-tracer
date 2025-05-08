use glam::{
    DVec2,
    DVec3,
};

use rand::{distr::uniform::SampleRange, Rng};

pub trait FromRng {
    fn from_rng<T: Rng>(rng: &mut T) -> Self;
    fn from_rng_ranged<T: Rng, R: SampleRange<f64> + Clone>(rng: &mut T, range: R) -> Self;
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

pub fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> DVec3 {
    loop {
        let p = DVec3::from_rng_ranged(rng, -1.0..1.0);
        let length_squared = p.length_squared();

        if 1e-160 < length_squared && length_squared <= 1.0 {
            break p/length_squared
        }
    }
}

pub fn random_on_hemisphere<T: Rng>(
    rng: &mut T,
    normal: DVec3,
) -> DVec3 {
    let on_unit_sphere = random_in_unit_sphere(rng);

    on_unit_sphere.dot(normal).signum()*on_unit_sphere
}

pub trait AlmostZero {
    fn almost_zero(&self, epsilon: f64) -> bool;
}

impl AlmostZero for DVec2 {
    fn almost_zero(&self, epsilon: f64) -> bool {
        self.x.abs() < epsilon && self.y.abs() < epsilon
    }
}

impl AlmostZero for DVec3 {
    fn almost_zero(&self, epsilon: f64) -> bool {
        self.x.abs() < epsilon &&
        self.y.abs() < epsilon &&
        self.z.abs() < epsilon
    }
}
