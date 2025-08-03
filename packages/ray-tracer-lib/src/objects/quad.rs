use glam::{DVec2, DVec3};

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename = "Quad")]
struct QuadConfig {
    top_left: DVec3,
    u: DVec3,
    v: DVec3,
    speed: Option<DVec3>,
    material: usize,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "QuadConfig")]
pub struct Quad {
    top_left: DVec3,
    u: DVec3,
    v: DVec3,
    speed: Option<DVec3>,
    material: usize,

    #[serde(skip)]
    normal: DVec3,

    #[serde(skip)]
    d: f64,

    #[serde(skip)]
    w: DVec3,

    #[serde(skip)]
    bbox: AABB,
}

impl From<QuadConfig> for Quad {
    fn from(data: QuadConfig) -> Self {
        Self::with_speed(data.top_left, data.speed, data.u, data.v, data.material)
    }
}

impl Quad {
    pub fn with_speed(
        top_left: DVec3,
        speed: Option<DVec3>,
        u: DVec3,
        v: DVec3,
        material: usize,
    ) -> Self {
        let bbox_t0 = AABB::from_points(top_left, top_left + u + v);
        let bbox_t1 = AABB::from_points(top_left + u, top_left + v);

        let bbox = bbox_t0.union(&bbox_t1);

        let n = u.cross(v);

        let normal = n.normalize();

        let d = normal.dot(top_left);
        let w = n/(n.dot(n));

        Self {
            top_left,
            speed,
            u,
            v,
            normal,
            d,
            w,
            material,
            bbox,
        }
    }

    pub fn new(
        top_left: DVec3,
        u: DVec3,
        v: DVec3,
        material: usize,
    ) -> Self {
        Self::with_speed(top_left, None, u, v, material)
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
        let material = self.material;

        Some(HitRecord::new_with_uv(ray, material, point, self.normal, uv, t))
    }
}
