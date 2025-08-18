use std::sync::Arc;

use glam::{
    DVec3,
    DMat3,
};

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct RotateY {
    object: Arc<dyn Hitable + Send + Sync>,
    rotation_mat: DMat3,
    rotation_mat_inv: DMat3,
    bbox: AABB,
}

impl RotateY {
    pub fn new(
        object: Arc<dyn Hitable + Send + Sync>,
        angle: f64,
    ) -> Self {
        let mut bbox = object.bbox();

        let mut min = DVec3::INFINITY.to_array();
        let mut max = DVec3::NEG_INFINITY.to_array();

        let cos_theta = f64::cos(angle);
        let sin_theta = f64::sin(angle);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64)*bbox.x.max + (1.0 - (i as f64))*bbox.x.min;
                    let y = (j as f64)*bbox.y.max + (1.0 - (j as f64))*bbox.y.min;
                    let z = (k as f64)*bbox.z.max + (1.0 - (k as f64))*bbox.z.min;

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = cos_theta*z - sin_theta*x;

                    let tester = DVec3::new(newx, y, newz);

                    for c in 0..=2 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        bbox = AABB::from_points(DVec3::from_array(min), DVec3::from_array(max));

        let rotation_mat = DMat3::from_axis_angle(DVec3::Y, -angle);
        let rotation_mat_inv = DMat3::from_axis_angle(DVec3::Y, angle);

        Self {
            object,
            rotation_mat,
            rotation_mat_inv,
            bbox,
        }
    }
}

impl Hitable for RotateY {
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
