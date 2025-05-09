use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vector::*;

use super::Material;

pub struct Lambertian {
    albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
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
