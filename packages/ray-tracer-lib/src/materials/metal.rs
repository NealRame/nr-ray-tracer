use std::sync::Arc;

use glam::DVec3;

use rand::RngCore;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::textures::{
    SolidColor,
    Texture,
};
use crate::vector::*;

use super::material::Material;

#[derive(Clone, Debug)]
pub struct Metal {
    fuzz: f64,
    texture: Arc<dyn Texture + Send + Sync>,
}

#[derive(Clone, Default)]
pub struct MetalBuilder {
    fuzz: Option<f64>,
    texture: Option<Arc<dyn Texture + Send + Sync>>,
}

impl MetalBuilder {
    pub fn with_fuzz(
        &mut self,
        value: Option<f64>,
    ) -> &mut Self {
        self.fuzz = value;
        self
    }

    pub fn with_color(
        &mut self,
        color: Option<DVec3>,
    ) -> &mut Self {
        self.texture = if let Some(color) = color {
             Some(Arc::new(SolidColor::new(color)))
        } else {
            None
        };
        self
    }

    pub fn with_texture(
        &mut self,
        texture: Option<Arc<dyn Texture + Send + Sync>>,
    ) -> &mut Self {
        self.texture = texture;
        self
    }

    pub fn build(self) -> Metal {
        Metal {
            fuzz: self.fuzz.unwrap_or(0.0),
            texture: self.texture.unwrap_or(Arc::new(SolidColor::default())),
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        MetalBuilder::default().build()
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Ray, DVec3)> {
        let scatter_direction =
            ray.get_direction().reflect(hit.normal).normalize()
                + self.fuzz*random_in_unit_sphere(rng);

        if scatter_direction.dot(hit.normal) > 0.0 {
            Some((
                Ray::new_at_time(hit.point, scatter_direction, ray.get_time()),
                self.texture.get_color(hit.texture_coordinates, hit.point),
            ))
        } else {
            None
        }
    }
}
