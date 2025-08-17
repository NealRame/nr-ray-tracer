use std::sync::Arc;

use rand::Rng;

use glam::{
    DVec2,
    DVec3,
    U64Vec2,
};

use crate::{prelude::SolidColor, vector::FromRng};

use super::texture::Texture;

#[derive(Clone)]
pub struct Checker {
    even_texture: Arc<dyn Texture + Send + Sync>,
    odd_texture: Arc<dyn Texture + Send + Sync>,
    scale: f64,
}

#[derive(Clone, Default)]
pub struct CheckerBuilder {
    even_texture: Option<Arc<dyn Texture + Send + Sync>>,
    odd_texture: Option<Arc<dyn Texture + Send + Sync>>,
    scale: Option<f64>,
}

impl CheckerBuilder {
    pub fn with_odd_texture(
        &mut self,
        texture: Option<Arc<dyn Texture + Send + Sync>>,
    ) -> &mut Self {
        self.odd_texture = texture;
        self
    }

    pub fn with_even_texture(
        &mut self,
        texture: Option<Arc<dyn Texture + Send + Sync>>,
    ) -> &mut Self {
        self.even_texture = texture;
        self
    }

    pub fn with_scale(&mut self, value: Option<f64>) -> &mut Self {
        self.scale = value;
        self
    }

    pub fn build(self) -> Checker {
        Checker {
            even_texture: self.even_texture.unwrap_or(Arc::new(SolidColor::new(DVec3::ONE))),
            odd_texture: self.odd_texture.unwrap_or(Arc::new(SolidColor::new(DVec3::ZERO))),
            scale: self.scale.unwrap_or(0.5)
        }
    }
}

impl Checker {
    pub fn from_rng<R: Rng>(rng: &mut R) -> Self {
        Self {
            even_texture: Arc::new(SolidColor::new(DVec3::from_rng_ranged(rng, 0.0..=1.0))),
            odd_texture: Arc::new(SolidColor::new(DVec3::from_rng_ranged(rng, 0.0..=1.0))),
            scale: rng.random_range(0.0..=1.0),
        }
    }
}

impl Default for Checker {
    fn default() -> Self {
        CheckerBuilder::default().build()
    }
}

impl Texture for Checker {
    fn get_color(
        &self,
        uv_coord: DVec2,
        point: DVec3,
    ) -> DVec3 {
        let v = (uv_coord*self.scale).as_u64vec2().dot(U64Vec2::ONE);

        if v%2 == 0 {
            self.even_texture.get_color(uv_coord, point)
        } else {
            self.odd_texture.get_color(uv_coord, point)
        }
    }
}
