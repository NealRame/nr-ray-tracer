use glam::DVec3;

use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, Default)]
pub struct HitRecord {
    pub front_face: bool,
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
    ) -> Self {
        let sign = ray.get_direction().dot(outward_normal).signum();

        let front_face = sign < 0.0;
        let normal = -sign*outward_normal;

        Self {
            front_face,
            point,
            normal,
            t,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord>;
}

pub struct HitableList {
    items: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        let items = Vec::default();
        Self { items }
    }

    pub fn add(
        &mut self,
        item: Box<dyn Hitable>,
    ) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn clear(
        &mut self,
    ) -> &mut Self {
        self.items.clear();
        self
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, mut hit_range: Interval) -> Option<HitRecord> {
        let mut hit_record = Option::<HitRecord>::None;

        for item in self.items.iter() {
            if let Some(rec) = item.hit(ray, hit_range.clone()) {
                hit_record.replace(rec);
                hit_range = hit_range.with_upper_bound(rec.t);
            }
        }

        hit_record
    }
}
