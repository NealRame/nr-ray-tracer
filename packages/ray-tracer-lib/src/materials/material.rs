use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::objects::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, DVec3)>;
}
