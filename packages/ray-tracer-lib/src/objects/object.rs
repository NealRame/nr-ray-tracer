use std::sync::Arc;

use serde::{
    Deserialize,
    Serialize,
};

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::ray::Ray;

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
    fn bbox(&self) -> AABB {
        match self {
            Self::Sphere(sphere) => sphere.bbox(),
        }
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray, hit_range),
        }
    }
}

impl Hitable for Vec<Object> {
    fn bbox(&self) -> AABB {
        self.iter().fold(AABB::EMPTY, |bbox, object| {
            bbox.union(&object.bbox())
        })
    }

    fn hit(
        &self,
        ray: &Ray,
        mut hit_range: Interval,
    ) -> Option<HitRecord> {
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

pub enum BVH {
    Leaf(Option<Object>),
    Node {
        bbox: AABB,
        left: Arc<BVH>,
        right: Arc<BVH>,
    },
}

impl BVH {
    pub fn from(objects: &mut [Object]) -> Self {
        match objects{
            [ ] =>  Self::Leaf(None),
            [o] =>  Self::Leaf(Some(o.clone())),
            [o1, o2] => {
                let left  = Arc::new(Self::Leaf(Some(o1.clone())));
                let right = Arc::new(Self::Leaf(Some(o2.clone())));
                let bbox  = AABB::union(&o1.bbox(), &o2.bbox());

                Self::Node { bbox, left, right }
            },
            _ => {
                let bbox = objects.iter().fold(AABB::EMPTY, |bbox, object| {
                    bbox.union(&object.bbox())
                });

                let axis = bbox.longest_axis();

                objects.sort_by(|o1, o2| {
                    let bb1_axis_interval = o1.bbox().axis_interval(axis).min;
                    let bb2_axis_interval = o2.bbox().axis_interval(axis).min;

                    f64::total_cmp(&bb1_axis_interval, &bb2_axis_interval)
                });

                let mid = objects.len()/2;
                let left  = Arc::new(BVH::from(&mut objects[..mid]));
                let right = Arc::new(BVH::from(&mut objects[mid..]));

                Self::Node { bbox, left, right }
            },
        }
    }
}

impl Hitable for BVH {
    fn bbox(&self) -> AABB {
        match self {
            Self::Node { bbox, .. } => {
                *bbox
            },
            Self::Leaf(Some(object)) => {
                object.bbox()
            },
            _ => AABB::EMPTY,
        }
    }

    fn hit(
        &self,
        ray: &Ray,
        hit_range: Interval,
    ) -> Option<HitRecord> {
        match self {
            Self::Leaf(Some(object)) => {
                object.hit(ray, hit_range)
            },
            Self::Node {
                bbox,
                left,
                right
            } if bbox.hit(ray, hit_range) => {
                match (
                    left.hit(ray, hit_range),
                    right.hit(ray, hit_range),
                ) {
                    (Some(hit_l), None) => Some(hit_l),
                    (None, Some(hit_r)) => Some(hit_r),
                    (Some(hit_l), Some(hit_r)) => {
                        if hit_l.t < hit_r.t {
                            Some(hit_l)
                        } else {
                            Some(hit_r)
                        }
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }
}
