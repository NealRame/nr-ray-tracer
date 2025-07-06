use std::f64::consts::PI;
use std::ops::Neg;

use glam::{
    DVec2,
    DVec3,
};

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use crate::aabb::AABB;
use crate::hitable::*;
use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename = "Sphere")]
struct SphereData {
    center: DVec3,
    speed: Option<DVec3>,
    radius: f64,
    material: Material,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "SphereData")]
pub struct Sphere {
    center: DVec3,
    speed: Option<DVec3>,
    radius: f64,
    material: Material,

    #[serde(skip)]
    bbox: AABB,
}

impl From<SphereData> for Sphere {
    fn from(data: SphereData) -> Self {
        Self::with_speed(data.center, data.speed, data.radius, data.material)
    }
}

impl Sphere {
    pub fn with_speed(
        center: DVec3,
        speed: Option<DVec3>,
        radius: f64,
        material: Material,
    ) -> Self {
        let rvec = DVec3::new(radius, radius, radius);

        let center_t0 = center;
        let center_t1 = center + speed.unwrap_or(DVec3::ZERO);

        let bbox_t0 = AABB::from_points(center_t0 - rvec, center_t0 + rvec);
        let bbox_t1 = AABB::from_points(center_t1 - rvec, center_t1 + rvec);
        let bbox = bbox_t0.union(&bbox_t1);

        Self {
            center,
            speed,
            radius,
            material,
            bbox,
        }
    }

    pub fn new(
        center: DVec3,
        radius: f64,
        material: Material,
    ) -> Self {
        Self::with_speed(center, None, radius, material)
    }
}

impl Hitable for Sphere {
    fn bbox(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let speed = Ray::new(self.center, self.speed.unwrap_or(DVec3::ZERO));
        let center = speed.at(ray.get_time());

        let dir = ray.get_direction();
        let eye = ray.get_origin();
        let ec = center - eye;

        let a = dir.length_squared();
        let h = ec.dot(dir);
        let c = ec.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        None
            .or_else(|| {
                let t = (h - sqrtd)/a;

                if hit_range.surrounds(t) {
                    Some(t)
                } else {
                    None
                }
            })
            .or_else(|| {
                let t = (h + sqrtd)/a;

                if hit_range.surrounds(t) {
                    Some(t)
                } else {
                    None
                }
            })
            .map(|t| {
                let point = ray.at(t);
                let normal = (point - center).normalize();
                let material = self.material.clone();

                let theta = f64::acos(normal.y.neg());
                let phi = f64::atan2(normal.z.neg(), normal.x) + PI;

                let uv = DVec2::new(
                    phi/(2.0*PI), // u
                    theta/PI,     // v
                );

                HitRecord::new_with_uv(ray, material, point, normal, uv, t)
            })
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use serde_test::{
        Token,
        assert_tokens,
    };

    use crate::materials::Material;
    use super::Sphere;

    #[test]
    fn test_sphere_serde() {
        let center = DVec3::new(1.0, 2.0, 3.0);
        let radius = 4.0;
        let material = Material::Dielectric { refraction_index: 1.42 };

        let sphere = Sphere::new(center, radius, material);
        assert_tokens(&sphere, &[
            Token::Struct { name: "Sphere", len: 3, },
            Token::Str("center"),
            Token::TupleStruct { name: "DVec3", len: 3, },
            Token::F64(1.0),
            Token::F64(2.0),
            Token::F64(3.0),
            Token::TupleStructEnd,
            Token::Str("radius"),
            Token::F64(4.0),
            Token::Str("material"),
            Token::Enum { name: "Material", },
            Token::Str("Dielectric"),
            Token::Map { len: Some(1), },
            Token::Str("refraction_index"),
            Token::F64(1.42),
            Token::MapEnd,
            Token::StructEnd,
        ]);
    }
}
