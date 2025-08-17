use glam::DVec3;

use rand::RngCore;

use crate::hitable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit: &HitRecord,
        _rng: &mut dyn RngCore
    ) -> Option<(Ray, DVec3)> {
        None
    }

    fn emit(
        &self,
        _hit: &HitRecord,
    ) -> DVec3 {
        DVec3::ZERO
    }
}
