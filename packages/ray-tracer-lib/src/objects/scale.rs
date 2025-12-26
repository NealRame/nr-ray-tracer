use std::sync::Arc;

use glam::{DMat4, DVec3};

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

fn scale_bbox(
    bbox: &AABB,
    scale_matrix: &DMat4,
) -> AABB {
    let mut min = DVec3::INFINITY;
    let mut max = DVec3::NEG_INFINITY;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let x = (i as f64)*bbox.x.max + (1.0 - (i as f64))*bbox.x.min;
                let y = (j as f64)*bbox.y.max + (1.0 - (j as f64))*bbox.y.min;
                let z = (k as f64)*bbox.z.max + (1.0 - (k as f64))*bbox.z.min;

                let tester = scale_matrix.transform_point3(DVec3::new(x, y, z));

                min = min.min(tester);
                max = max.max(tester);
            }
        }
    }

    AABB::from_points(min, max)
}

#[derive(Clone, Debug)]
pub struct Scale {
    object: Arc<dyn Hitable + Send + Sync>,
    scale_matrix: DMat4,
    scale_matrix_inv: DMat4,
    bbox: AABB,
}

impl Scale {
    pub fn new(
        object: Arc<dyn Hitable + Send + Sync>,
        scale: DVec3,
    ) -> Self {
        let scale_matrix = DMat4::from_scale(scale);
        let scale_matrix_inv = scale_matrix.inverse();
        let bbox =  scale_bbox(&object.bbox(), &scale_matrix);

        Self {
            object,
            scale_matrix,
            scale_matrix_inv,
            bbox,
        }
    }

    pub fn uniform(
        object: Arc<dyn Hitable + Send + Sync>,
        factor: f64,
    ) -> Self {
        Self::new(object, factor*DVec3::ONE)
    }
}

impl Hitable for Scale {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let scaled_ray = Ray::new_at_time(
             self.scale_matrix_inv.transform_point3(ray.get_origin()),
             self.scale_matrix_inv.transform_vector3(ray.get_direction()),
             ray.get_time(),
        );

        self.object
            .hit(&scaled_ray, hit_range)
            .map(|mut hit| {
                hit.point = self.scale_matrix.transform_point3(hit.point);
                hit
            })
    }
}
