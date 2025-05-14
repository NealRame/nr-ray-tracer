use glam::DVec3;

use rand::{rngs::ThreadRng, Rng};

use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::*;

use super::Material;

pub struct Lambertian {
    pub albedo: DVec3,
}

impl Lambertian {
    pub fn from_rng(rng: &mut ThreadRng) -> Self {
        Self {
            albedo: DVec3::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
            )
        }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: DVec3::ONE/2.,
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, DVec3)> {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere(rng);

        if scatter_direction.almost_zero(1e-8) {
            scatter_direction = hit_record.normal
        }

        Some((
            Ray::new(hit_record.point, scatter_direction),
            self.albedo,
        ))
    }
}
