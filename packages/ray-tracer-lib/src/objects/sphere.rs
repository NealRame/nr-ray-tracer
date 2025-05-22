use glam::DVec3;

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use super::hitable::{
    HitRecord,
    Hitable,
};

use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;

#[skip_serializing_none]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Sphere {
    center: DVec3,
    speed: Option<DVec3>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn with_speed(
        center: DVec3,
        speed: Option<DVec3>,
        radius: f64,
        material: Material,
    ) -> Self {
        Self {
            center,
            speed,
            radius,
            material,
        }
    }

    pub fn new(
        center: DVec3,
        radius: f64,
        material: Material,
    ) -> Self {
        Self::with_speed(center, None, radius, material)
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let speed = Ray::new(self.center, self.speed.unwrap_or(DVec3::ZERO));
        let center = speed.at(ray.get_time());

        let dir = ray.get_direction();
        let eye = ray.get_origin();
        let ec = center - eye;

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
                let normal = (point - center).normalize();
                let material = self.material.clone();

                HitRecord::new(ray, material, point, normal, t)
            })
    }
}
