use std::sync::Arc;

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Scale {
    object: Arc<dyn Hitable + Send + Sync>,
    factor: f64,
    bbox: AABB,
}

impl Scale {
    pub fn new(
        object: Arc<dyn Hitable + Send + Sync>,
        factor: f64,
    ) -> Self {
        let bbox = object.bbox().scaled(factor);

        Self {
            object,
            factor,
            bbox,
        }
    }
}

impl Hitable for Scale {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let scaled_ray = Ray::new_at_time(
            ray.get_origin()/self.factor,
            ray.get_direction()/self.factor,
            ray.get_time(),
        );

        self.object
            .hit(&scaled_ray, hit_range)
            .map(|mut hit| {
                hit.point *= self.factor;
                hit
            })
    }
}
