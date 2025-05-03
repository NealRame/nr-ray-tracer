use glam::DVec3;

use crate::ray::Ray;

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, hit_range: std::ops::Range<f64>) -> Option<HitRecord>;
}
