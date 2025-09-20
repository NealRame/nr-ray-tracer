use std::sync::Arc;
use std::f64::consts::PI;
use std::ops::Neg;

use glam::{
    DVec2,
    DVec3,
};

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::materials::{
    Lambertian,
    Material,
};
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: DVec3,
    speed: Option<DVec3>,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
    bbox: AABB,
}

#[derive(Default)]
pub struct SphereBuilder {
    center: Option<DVec3>,
    speed: Option<DVec3>,
    radius: Option<f64>,
    material: Option<Arc<dyn Material + Send + Sync>>,
}

impl SphereBuilder {
    pub fn with_center(
        &mut self,
        value: DVec3,
    ) -> &mut Self {
        self.center.replace(value);
        self
    }

    pub fn with_speed(
        &mut self,
        value: DVec3,
    ) -> &mut Self {
        self.speed.replace(value);
        self
    }

    pub fn with_radius(
        &mut self,
        value: f64,
    ) -> &mut Self {
        self.radius.replace(value);
        self
    }

    pub fn with_material(
        &mut self,
        value: Arc<dyn Material + Send + Sync>,
    ) -> &mut Self {
        self.material.replace(value);
        self
    }

    pub fn build(self) -> Sphere {
        let center = self.center.unwrap_or(DVec3::ZERO);
        let radius = self.radius.unwrap_or(0.);
        let rvec = DVec3::new(radius, radius, radius);
        let speed = self.speed;

        let center_t0 = center;
        let center_t1 = center + speed.unwrap_or(DVec3::ZERO);

        let bbox_t0 = AABB::from_points(center_t0 - rvec, center_t0 + rvec);
        let bbox_t1 = AABB::from_points(center_t1 - rvec, center_t1 + rvec);
        let bbox = bbox_t0.union(&bbox_t1);

        let material = self.material.unwrap_or(Arc::new(Lambertian::default()));

        Sphere {
            center,
            speed,
            radius,
            material,
            bbox,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        SphereBuilder::default().build()
    }
}

impl Hitable for Sphere {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(
        &self,
        ray: &Ray,
        hit_range: Interval,
    ) -> Option<HitRecord> {
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
                let material = self.material.clone();
                let normal = (point - center).normalize();

                let theta = f64::acos(normal.y.neg());
                let phi = f64::atan2(normal.z.neg(), normal.x) + PI;

                let uv = DVec2::new(
                    phi/(2.0*PI), // u
                    theta/PI,     // v
                );

                HitRecord::new_with_uv(ray, material, point, normal, uv, t)
            })
    }
}
