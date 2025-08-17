use rand::Rng;

use glam::{
    DVec2,
    DVec3,
};

use crate::vector::FromRng;

use super::texture::Texture;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolidColor {
    color: DVec3,
}

impl SolidColor {
    pub fn new(color: DVec3) -> Self {
        Self {
            color,
        }
    }

    pub fn from_rng<R: Rng>(rng: &mut R) -> Self {
        Self::new(DVec3::from_rng_ranged(rng, 0.0..=1.0))
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        Self::new(DVec3::ONE)
    }
}

impl Texture for SolidColor {
    fn get_color(
        &self,
        _: DVec2,
        _: DVec3,
    ) -> DVec3 {
        self.color
    }
}
