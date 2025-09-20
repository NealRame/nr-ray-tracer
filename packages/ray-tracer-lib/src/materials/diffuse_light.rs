use std::sync::Arc;

use glam::DVec3;

use crate::hitable::HitRecord;
use crate::prelude::SolidColor;
use crate::textures::Texture;

use super::material::Material;

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    texture: Arc<dyn Texture + Send + Sync>,
}

impl Default for DiffuseLight {
    fn default() -> Self {
        Self {
            texture: Arc::new(SolidColor::default())
        }
    }
}

impl DiffuseLight {
    pub fn with_color(color: DVec3) -> Self {
        Self::with_texture(Arc::new(SolidColor::new(color)))
    }

    pub fn with_texture(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn emit(
        &self,
        hit: &HitRecord,
    ) -> DVec3 {
        self.texture.get_color(hit.texture_coordinates, hit.point)
    }
}
