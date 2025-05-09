use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vector::*;

use super::Material;

pub struct Metal {
    albedo: DVec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, DVec3)> {
        let reflected_direction =
            ray.get_direction().reflect(hit_record.normal).normalize()
                + self.fuzz*random_in_unit_sphere(rng);

        if reflected_direction.dot(hit_record.normal) > 0.0 {
            Some((
                Ray::new(hit_record.point, reflected_direction),
                self.albedo,
            ))
        } else {
            None
        }
    }
}
