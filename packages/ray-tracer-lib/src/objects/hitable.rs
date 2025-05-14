use glam::DVec3;

use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub front_face: bool,
    pub material: Material,
    pub normal: DVec3,
    pub point: DVec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        material: Material,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
    ) -> Self {
        let sign = ray.get_direction().dot(outward_normal).signum();

        let front_face = sign < 0.0;
        let normal = -sign*outward_normal;

        Self {
            front_face,
            material,
            normal,
            point,
            t,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord>;
}
