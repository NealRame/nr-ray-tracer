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
    fuzz: f64,
    rng: &mut T,
) -> Option<(Ray, DVec3)> {
    let scatter_direction =
        ray.get_direction().reflect(hit_record.normal).normalize()
            + fuzz*random_in_unit_sphere(rng);

    if scatter_direction.dot(hit_record.normal) > 0.0 {
        Some((
            Ray::new_at_time(hit_record.point, scatter_direction, ray.get_time()),
            texture.get_color(hit_record.texture_coordinates, hit_record.point),
        ))
    } else {
        None
    }
}
