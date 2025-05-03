use glam::DVec3;

use crate::hitable::{
    HitRecord,
    Hitable,
};

use crate::interval::Interval;
use crate::ray::Ray;

pub struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let dir = ray.get_direction();
        let eye = ray.get_origin();
        let ec = self.center - eye;

        let a = dir.length_squared();
        let h = ec.dot(dir);
        let c = ec.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        None
            .or_else(|| {
                let t = (h - sqrtd)/a;

                if hit_range.surrounds(t) {
                    Some(t)
                } else {
                    None
                }
            })
            .or_else(|| {
                let t = (h + sqrtd)/a;

                if hit_range.surrounds(t) {
                    Some(t)
                } else {
                    None
                }
            })
            .map(|t| {
                let point = ray.at(t);
                let normal = (point - self.center).normalize();

                HitRecord::new(ray, point, normal, t)
            })
    }
}
