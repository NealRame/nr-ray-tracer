use glam::DVec3;

use rand::rngs::ThreadRng;

use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::*;

// #[derive(Clone, Copy, Debug)]
// pub struct Metal {
//     pub albedo: DVec3,
//     pub fuzz: f64,
// }

// impl Metal {
//     pub fn from_rng(rng: &mut ThreadRng) -> Self {
//         Self {
//             albedo: DVec3::new(
//                 rng.random_range(0.0..=1.0),
//                 rng.random_range(0.0..=1.0),
//                 rng.random_range(0.0..=1.0),
//             ),
//             fuzz: rng.random_range(0.0..=1.0),
//         }
//     }
// }

// impl Default for Metal {
//     fn default() -> Self {
//         Self {
//             albedo: DVec3::ONE/2.,
//             fuzz: 0.0,
//         }
//     }
// }

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
