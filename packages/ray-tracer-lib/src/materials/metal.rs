use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::*;

pub(super) fn scatter(
    ray: &Ray,
    hit_record: &HitRecord,
    rng: &mut ThreadRng,
    albedo: DVec3,
    fuzz: f64,
) -> Option<(Ray, DVec3)> {
    let scatter_direction =
        ray.get_direction().reflect(hit_record.normal).normalize()
            + fuzz*random_in_unit_sphere(rng);

    if scatter_direction.dot(hit_record.normal) > 0.0 {
        Some((
            Ray::new_at_time(hit_record.point, scatter_direction, ray.get_time()),
            albedo,
        ))
    } else {
        None
    }
}
