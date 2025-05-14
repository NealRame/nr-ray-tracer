use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::*;

pub(super) fn scatter(
    albedo: DVec3,
    hit_record: &HitRecord,
    rng: &mut ThreadRng,
) -> Option<(Ray, DVec3)> {
    let mut scatter_direction = hit_record.normal + random_in_unit_sphere(rng);

    if scatter_direction.almost_zero(1e-8) {
        scatter_direction = hit_record.normal
    }

    Some((
        Ray::new(hit_record.point, scatter_direction),
        albedo,
    ))
}
