use glam::{
    DVec2,
    DVec3,
};

use rand::{
    Rng,
    RngCore,
};
use rand::distr::uniform::SampleRange;

pub trait FromRng {
    fn from_rng(rng: &mut dyn RngCore) -> Self;
    fn from_rng_ranged<R: SampleRange<f64> + Clone>(rng: &mut dyn RngCore, range: R) -> Self;
}

impl FromRng for DVec2 {
    fn from_rng(
        rng: &mut dyn RngCore,
    ) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
        }
    }

    fn from_rng_ranged<R: SampleRange<f64> + Clone>(
        rng: &mut dyn RngCore,
        range: R,
    ) -> Self {
        Self {
            x: rng.random_range(range.clone()),
            y: rng.random_range(range.clone()),
        }
    }
}

impl FromRng for DVec3 {
    fn from_rng(
        rng: &mut dyn RngCore,
    ) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }

    fn from_rng_ranged<R: SampleRange<f64> + Clone>(
        rng: &mut dyn RngCore,
        range: R,
    ) -> Self {
        Self {
            x: rng.random_range(range.clone()),
            y: rng.random_range(range.clone()),
            z: rng.random_range(range.clone()),
        }
    }
}

pub fn random_in_unit_sphere(rng: &mut dyn RngCore) -> DVec3 {
    loop {
        let p = DVec3::from_rng_ranged(rng, -1.0..1.0);
        let length_squared = p.length_squared();

        if 1e-160 < length_squared && length_squared <= 1.0 {
            break p/length_squared
        }
    }
}

pub fn random_in_unit_disk(rng: &mut dyn RngCore) -> DVec3 {
    loop {
        let p = DVec2::from_rng_ranged(rng, -1.0..1.0).extend(0.0);
        let length_squared = p.length_squared();

        if length_squared < 1.0 {
            break p/length_squared
        }
    }
}

pub fn random_on_hemisphere(
    rng: &mut dyn RngCore,
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
