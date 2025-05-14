use serde::{
    Deserialize,
    Serialize,
};

use crate::interval::Interval;
use crate::ray::Ray;

use super::*;
use super::Sphere;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Object {
    Sphere(Sphere)
}

impl From<Sphere> for Object {
    fn from(value: Sphere) -> Self {
        Self::Sphere(value)
    }
}

impl Hitable for Object {
    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray, hit_range),
        }
    }
}

impl Hitable for Vec<Object> {
    fn hit(
        &self,
        ray: &Ray,
        mut hit_range: Interval,
    ) -> Option<super::HitRecord> {
        let mut hit_record = Option::<HitRecord>::None;

        for obj in self.iter() {
            if let Some(rec) = obj.hit(ray, hit_range.clone()).as_ref() {
                hit_record.replace(rec.clone());
                hit_range = hit_range.with_upper_bound(rec.t);
            }
        }

        hit_record
    }
}
