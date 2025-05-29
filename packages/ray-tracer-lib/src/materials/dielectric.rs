use glam::DVec3;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::hitable::HitRecord;
use crate::ray::Ray;

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0 - refraction_index)/(1.0 + refraction_index);

    r0 = r0*r0;
    r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
}

pub(super) fn scatter(
    ray: &Ray,
    hit_record: &HitRecord,
    rng: &mut ThreadRng,
    refraction_index: f64,
) -> Option<(Ray, DVec3)> {
    let ri = if hit_record.front_face {
        1.0/refraction_index
    } else {
        refraction_index
    };

    let unit_direction = ray.get_direction().normalize();

    let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

    let scatter_direction =
        if ri*sin_theta > 1.0 || reflectance(cos_theta, ri) > rng.random_range(0.0..1.0) {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, ri)
        };

    Some((
        Ray::new_at_time(hit_record.point, scatter_direction, ray.get_time()),
        DVec3::ONE,
    ))
}
