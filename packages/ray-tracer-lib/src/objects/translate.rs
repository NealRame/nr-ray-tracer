use std::sync::Arc;

use glam::DVec3;

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hitable + Send + Sync>,
    offset: DVec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(
        object: Arc<dyn Hitable + Send + Sync>,
        offset: DVec3,
    ) -> Self {
        let bbox = object.bbox().translated(offset);

        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hitable for Translate {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let translated_ray = Ray::new_at_time(
            ray.get_origin() - self.offset,
            ray.get_direction(),
            ray.get_time(),
        );

        self.object.hit(&translated_ray, hit_range)
            .map(|mut hit| {
                hit.point += self.offset;
                hit
            })
    }
}
