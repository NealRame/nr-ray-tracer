use glam::DVec3;

use rand::Rng;
use rand::rngs::ThreadRng;

use serde::Deserialize;
use serde::Serialize;

use crate::hitable::HitRecord;
use crate::ray::Ray;

use crate::textures::Texture;

use super::dielectric;
use super::lambertian;
use super::metal;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Material {
    Dielectric { refraction_index: f64 },
    Lambertian { texture: Texture },
    Metal      { texture: Texture, fuzz: f64 },
}

impl Material {
    pub fn dielectric_default() -> Self {
        Self::Dielectric { refraction_index: 1.5 }
    }

    pub fn lambertian_default() -> Self {
        Self::Lambertian {
            texture: Texture::default_solid_color(),
        }
    }

    pub fn metal_default() -> Self {
        Self::Metal {
            texture: Texture::default_solid_color(),
            fuzz: 0.0,
        }
    }
}

impl Material {
    pub fn dielectric_from_rng(rng: &mut ThreadRng) -> Self {
        Self::Dielectric {
            refraction_index: rng.random_range(0.5..2.0),
        }
    }

    pub fn lambertian_from_rng(rng: &mut ThreadRng) -> Self {
        Self::Lambertian {
            texture: Texture::solid_color_from_rng(rng),
        }
    }

    pub fn metal_from_rng(rng: &mut ThreadRng) -> Self {
        Self::Metal{
            texture: Texture::solid_color_from_rng(rng),
            fuzz: rng.random_range(0.0..=1.0),
        }
    }
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, DVec3)> {
        match self {
            Self::Dielectric { refraction_index } => {
                dielectric::scatter(ray, hit_record, rng, *refraction_index, )
            },
            Self::Lambertian { texture } => {
                lambertian::scatter(ray, hit_record, texture, rng)
            },
            Self::Metal { texture, fuzz } => {
                metal::scatter(ray, hit_record, texture, *fuzz, rng)
            },
        }
    }
}
