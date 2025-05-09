use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::hitable::HitRecord;

pub trait Material {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut ThreadRng,
    ) -> Option<(Ray, DVec3)> {
        None
    }
}
