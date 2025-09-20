use std::sync::Arc;

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
pub struct Quad {
    top_left: DVec3,
    u: DVec3,
    v: DVec3,
    material: Arc<dyn Material + Send + Sync>,
    normal: DVec3,
    d: f64,
    w: DVec3,
    bbox: AABB,
}

#[derive(Clone, Default)]
pub struct QuadBuilder {
    top_left: Option<DVec3>,
    u: Option<DVec3>,
    v: Option<DVec3>,
    material: Option<Arc<dyn Material + Send + Sync>>,
}

impl QuadBuilder {
    pub fn with_top_left(
        &mut self,
        value: DVec3,
    ) -> &mut Self {
        self.top_left.replace(value);
        self
    }

    pub fn with_u(
        &mut self,
        value: DVec3,
    ) -> &mut Self {
        self.u.replace(value);
        self
    }

    pub fn with_v(
        &mut self,
        value: DVec3,
    ) -> &mut Self {
        self.v.replace(value);
        self
    }

    pub fn with_material(
        &mut self,
        value: Arc<dyn Material + Send + Sync>,
    ) -> &mut Self {
        self.material.replace(value);
        self
    }

    pub fn build(self) -> Quad {
        let top_left = self.top_left.unwrap_or(DVec3::ZERO);
        let u = self.u.unwrap_or(DVec3::X);
        let v = self.v.unwrap_or(DVec3::Y);
        let material = self.material.unwrap_or(Arc::new(Lambertian::default()));

        let bbox_t0 = AABB::from_points(top_left, top_left + u + v);
        let bbox_t1 = AABB::from_points(top_left + u, top_left + v);

        let bbox = bbox_t0.union(&bbox_t1);

        let n = u.cross(v);

        let normal = n.normalize();

        let d = normal.dot(top_left);
        let w = n/(n.dot(n));

        Quad {
            top_left,
            u,
            v,
            normal,
            d,
            w,
            material,
            bbox,
        }
    }
}

impl Default for Quad {
    fn default() -> Self {
        QuadBuilder::default().build()
    }
}

impl Hitable for Quad {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.get_direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.get_origin()))/denom;

        if !hit_range.contains(t) {
            return None;
        }

        let point = ray.at(t);
        let planar_hit_vector = point - self.top_left;

        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        let unit_interval = 0.0..=1.0;

        if !unit_interval.contains(&alpha) || !unit_interval.contains(&beta) {
            return None;
        }

        let uv = DVec2::new(alpha, beta);
        let material = self.material.clone();

        Some(HitRecord::new_with_uv(ray, material, point, self.normal, uv, t))
    }
}
