use glam::DVec3;

use rand::Rng;
use rand::rngs::ThreadRng;

use serde::Deserialize;
use serde::Serialize;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::scene::Scene;

use super::dielectric;
use super::lambertian;
use super::metal;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Material {
    Dielectric   { refraction_index: f64, },
    DiffuseLight { texture: usize, },
    Lambertian   { texture: usize, },
    Metal        { texture: usize, fuzz: f64 },
}

impl Material {
    pub fn dielectric_default() -> Self {
        Self::Dielectric { refraction_index: 1.5 }
    }

    pub fn lambertian_default() -> Self {
        Self::Lambertian {
            texture: 0,
        }
    }

    pub fn metal_default() -> Self {
        Self::Metal {
            texture: 0,
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
}

impl Material {
    pub fn scatter<T: Rng>(
        &self,
        scene: &Scene,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut T,
    ) -> Option<(Ray, DVec3)> {
        match self {
            Self::Dielectric { refraction_index } => {
                dielectric::scatter(ray, hit_record, rng, *refraction_index, )
            },
            Self::Lambertian { texture } => {
                let texture = scene.textures.get(*texture).expect("texture not found");
                lambertian::scatter(ray, hit_record, texture, rng)
            },
            Self::Metal { texture, fuzz } => {
                let texture = scene.textures.get(*texture).expect("texture not found");
                metal::scatter(ray, hit_record, texture, *fuzz, rng)
            },
            _ => None
        }
    }

    pub fn emit(
        &self,
        scene: &Scene,
        hit_record: &HitRecord,
    ) -> DVec3 {
        match self {
            Self::DiffuseLight { texture } => {
                let texture = scene.textures.get(*texture).expect("texture not found");
                texture.get_color(
                    hit_record.texture_coordinates,
                    hit_record.point,
                )
            },
            _ => DVec3::ZERO,
        }
    }
}
