use std::sync::Arc;

use glam::DVec3;

use crate::prelude::{
    HitRecord,
    Ray,
    SolidColor,
    Texture,
};

use super::material::Material;

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    intensity: f64,
    texture: Arc<dyn Texture + Send + Sync>,
}

#[derive(Clone, Debug, Default)]
pub struct DiffuseLightBuilder {
    intensity: Option<f64>,
    texture: Option<Arc<dyn Texture + Send + Sync>>,
}

impl DiffuseLightBuilder {
    pub fn with_color(&mut self, color: DVec3) -> &mut Self {
        self.texture.replace(Arc::new(SolidColor::new(color)));
        self
    }

    pub fn with_texture(
        &mut self,
        texture: Arc<dyn Texture + Send + Sync>,
    ) -> &mut Self {
        self.texture.replace(texture);
        self
    }

    pub fn with_intensity(
        &mut self,
        intensity: f64,
    ) -> &mut Self {
        self.intensity.replace(intensity);
        self
    }

    pub fn build(
        &self,
    ) -> DiffuseLight {
        let intensity = self.intensity.unwrap_or(4.0);
        let texture = if let Some(texture) = &self.texture {
            texture.clone()
        } else {
            Arc::new(SolidColor::default())
        };

        DiffuseLight { intensity, texture }
    }
}

impl Material for DiffuseLight {
    fn emit(
        &self,
        ray: &Ray,
        hit: &HitRecord,
    ) -> DVec3 {
        let k = if ray.get_bounce() > 0 {
            self.intensity
        } else {
            1.0
        };

        k*self.texture.get_color(hit.texture_coordinates, hit.point)
    }
}
