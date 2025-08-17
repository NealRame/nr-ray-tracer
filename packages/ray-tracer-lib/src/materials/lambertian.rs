use std::sync::Arc;

use glam::DVec3;

use rand::RngCore;

use crate::hitable::HitRecord;
use crate::prelude::SolidColor;
use crate::ray::Ray;
use crate::textures::Texture;
use crate::vector::*;

use super::material::Material;

pub struct Lambertian {
    texture: Arc<dyn Texture + Send + Sync>,
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            texture: Arc::new(SolidColor::default())
        }
    }
}

impl Lambertian {
    pub fn with_color(color: DVec3) -> Self {
        Self::with_texture(Arc::new(SolidColor::new(color)))
    }

    pub fn with_texture(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut dyn RngCore
    ) -> Option<(Ray, DVec3)> {
        let mut scatter_direction = hit.normal + random_in_unit_sphere(rng);

        if scatter_direction.almost_zero(1e-8) {
            scatter_direction = hit.normal
        }

        Some((
            Ray::new_at_time(hit.point, scatter_direction, ray.get_time()),
            self.texture.get_color(hit.texture_coordinates, hit.point),
        ))
    }
}
