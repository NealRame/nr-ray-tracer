use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::*;

pub(super) fn scatter(
    albedo: DVec3,
    fuzz: f64,
    ray: &Ray,
    hit_record: &HitRecord,
    rng: &mut ThreadRng,
) -> Option<(Ray, DVec3)> {
    let reflected_direction =
        ray.get_direction().reflect(hit_record.normal).normalize()
            + fuzz*random_in_unit_sphere(rng);

    if reflected_direction.dot(hit_record.normal) > 0.0 {
        Some((
            Ray::new(hit_record.point, reflected_direction),
            albedo,
        ))
    } else {
        None
    }
}
