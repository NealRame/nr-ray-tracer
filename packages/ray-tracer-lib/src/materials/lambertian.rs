use glam::DVec3;

use rand::Rng;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::textures::Texture;
use crate::vector::*;

pub(super) fn scatter<T: Rng>(
    ray: &Ray,
    hit_record: &HitRecord,
    texture: &Texture,
    rng: &mut T,
) -> Option<(Ray, DVec3)> {
    let mut scatter_direction = hit_record.normal + random_in_unit_sphere(rng);

    if scatter_direction.almost_zero(1e-8) {
        scatter_direction = hit_record.normal
    }

    Some((
        Ray::new_at_time(hit_record.point, scatter_direction, ray.get_time()),
        texture.get_color(hit_record.texture_coordinates, hit_record.point),
    ))
}
