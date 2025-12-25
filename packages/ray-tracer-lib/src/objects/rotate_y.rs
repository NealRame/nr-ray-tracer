use std::sync::Arc;

use glam::{
    DVec3,
    DMat3,
};

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

fn rotate_bbox(
    bbox: &AABB,
    rotation_mat: &DMat3,
) -> AABB {
    let mut min = DVec3::INFINITY;
    let mut max = DVec3::NEG_INFINITY;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let x = (i as f64)*bbox.x.max + (1.0 - (i as f64))*bbox.x.min;
                let y = (j as f64)*bbox.y.max + (1.0 - (j as f64))*bbox.y.min;
                let z = (k as f64)*bbox.z.max + (1.0 - (k as f64))*bbox.z.min;

                let tester = rotation_mat*DVec3::new(x, y, z);

                min = min.min(tester);
                max = max.max(tester);
            }
        }
    }

    AABB::from_points(min, max)
}

#[derive(Clone, Debug)]
pub struct Rotate {
    object: Arc<dyn Hitable + Send + Sync>,
    rotation_mat: DMat3,
    rotation_mat_inv: DMat3,
    bbox: AABB,
}

impl Rotate {
    fn new(
        object: Arc<dyn Hitable + Send + Sync>,
        axis: DVec3,
        angle: f64,
    ) -> Self {
        let rotation_mat = DMat3::from_axis_angle(axis, -angle);
        let rotation_mat_inv = DMat3::from_axis_angle(axis, angle);
        let bbox = rotate_bbox(&object.bbox(), &rotation_mat);

        Self {
            object,
            rotation_mat,
            rotation_mat_inv,
            bbox,
        }
    }

    pub fn axis_x(
        object: Arc<dyn Hitable + Send + Sync>,
        angle: f64,
    ) -> Self {
        Self::new(object, DVec3::X, angle)
    }

    pub fn axis_y(
        object: Arc<dyn Hitable + Send + Sync>,
        angle: f64,
    ) -> Self {
        Self::new(object, DVec3::Y, angle)
    }

    pub fn axis_z(
        object: Arc<dyn Hitable + Send + Sync>,
        angle: f64,
    ) -> Self {
        Self::new(object, DVec3::Z, angle)
    }
}

impl Hitable for Rotate {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let rotated_origin = self.rotation_mat*ray.get_origin();
        let rotated_direction = self.rotation_mat*ray.get_direction();
        let rotated_ray = Ray::new_at_time(
            rotated_origin,
            rotated_direction,
            ray.get_time()
        );

        self.object.hit(&rotated_ray, hit_range)
            .map(|mut hit| {
                hit.point = self.rotation_mat_inv*hit.point;
                hit.normal = self.rotation_mat_inv*hit.normal;
                hit
            })
    }
}
