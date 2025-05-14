use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::hitable::HitRecord;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, DVec3)>;
}
