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
pub enum Shape {
    Quad,
    Triangle,
}

fn is_interior_quad(alpha: f64, beta: f64) -> bool {
    let unit_interval = 0.0..=1.0;
    unit_interval.contains(&alpha) && unit_interval.contains(&beta)
}

fn is_interior_triangle(alpha: f64, beta: f64) -> bool {
    alpha > 0.0 && beta > 0.0 && (alpha + beta) < 1.0
}

#[derive(Clone, Debug)]
pub struct Plane {
    p: DVec3,
    u: DVec3,
    v: DVec3,
    shape: Shape,
    bbox: AABB,
    material: Arc<dyn Material + Send + Sync>,
    normal: DVec3,
    d: f64,
    w: DVec3,
}

#[derive(Clone, Default)]
pub struct PlaneBuilder {
    p: Option<DVec3>,
    u: Option<DVec3>,
    v: Option<DVec3>,
    shape: Option<Shape>,
    material: Option<Arc<dyn Material + Send + Sync>>,
}

impl PlaneBuilder {
    pub fn with_point(
        &mut self,
        value: DVec3,
    ) -> &mut Self {
        self.p.replace(value);
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

    pub fn with_shape(
        &mut self,
        value: Shape,
    ) -> &mut Self {
        self.shape.replace(value);
        self
    }

    pub fn with_material(
        &mut self,
        value: Arc<dyn Material + Send + Sync>,
    ) -> &mut Self {
        self.material.replace(value);
        self
    }

    pub fn build(self) -> Plane {
        let p = self.p.unwrap_or(DVec3::ZERO);
        let u = self.u.unwrap_or(DVec3::X);
        let v = self.v.unwrap_or(DVec3::Y);

        let shape = self.shape.unwrap_or(Shape::Quad);

        let material = self.material.unwrap_or(Arc::new(Lambertian::default()));

        let bbox_t0 = AABB::from_points(p, p + u + v);
        let bbox_t1 = AABB::from_points(p + u, p + v);

        let bbox = bbox_t0.union(&bbox_t1);

        let n = u.cross(v);

        let normal = n.normalize();

        let d = normal.dot(p);
        let w = n/(n.dot(n));

        Plane {
            p,
            u,
            v,
            shape,
            normal,
            d,
            w,
            material,
            bbox,
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        PlaneBuilder::default().build()
    }
}

impl Hitable for Plane {
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
        let planar_hit_vector = point - self.p;

        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        let is_interior =
            match self.shape {
                Shape::Quad => is_interior_quad,
                Shape::Triangle => is_interior_triangle,
            };

        if !is_interior(alpha, beta) {
            return None;
        }

        let uv = DVec2::new(alpha, beta);
        let material = self.material.clone();

        Some(HitRecord::new_with_uv(ray, material, point, self.normal, uv, t))
    }
}
