use glam::{
    DVec2,
    DVec3,
};

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub front_face: bool,
    pub material: usize,
    pub normal: DVec3,
    pub point: DVec3,
    pub t: f64,
    pub texture_coordinates: DVec2,
}

impl HitRecord {
    pub fn new_with_uv(
        ray: &Ray,
        material: usize,
        point: DVec3,
        outward_normal: DVec3,
        texture_coordinates: DVec2,
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
            texture_coordinates,
            t,
        }
    }

    pub fn new(
        ray: &Ray,
        material: usize,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
    ) -> Self {
        Self::new_with_uv(
            ray,
            material,
            point,
            outward_normal,
            DVec2::default(),
            t,
        )
    }
}

pub trait Hitable {
    fn bbox(&self) -> AABB;
    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord>;
}
