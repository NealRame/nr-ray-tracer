use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{
    anyhow,
    Result,
};

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use serde::{
    Deserialize,
    Serialize,
};

use crate::cli::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum TextureConfig {
    Checker {
        even: Option<usize>,
        odd: Option<usize>,
        scale: Option<f64>,
    },
    Image {
        path: PathBuf
    },
    Marble {
        seed: Option<u32>,
        frequency: Option<f64>,
    },
    Noise {
        seed: Option<u32>,
        frequency: Option<f64>,
        lacunarity: Option<f64>,
        octaves: Option<usize>,
        persistence: Option<f64>,
    },
    SolidColor {
        color: DVec3,
    }
}

impl TextureConfig {
    pub fn try_make_texture(
        &self,
        textures: &[Arc<dyn Texture + Send + Sync>],
    ) -> Result<Arc<dyn Texture + Send + Sync>> {
        match self {
            Self::Checker {
                scale,
                even,
                odd,
            } => {
                let mut checker_builder = CheckerBuilder::default();

                if let Some(even) = *even {
                    checker_builder.with_even_texture(Some(
                        textures
                            .get(even)
                            .ok_or(anyhow!("invalid texture index"))?
                            .clone()
                    ));
                }

                if let Some(odd) = *odd {
                    checker_builder.with_odd_texture(Some(
                        textures
                            .get(odd)
                            .ok_or(anyhow!("invalid texture index"))?
                            .clone()
                    ));
                }

                checker_builder.with_scale(*scale);

                Ok(Arc::new(checker_builder.build()))
            },
            Self::Image { path } => {
                Ok(Arc::new(Image::try_from_path(path)?))
            },
            Self::Marble {
                seed,
                frequency,
            } => {
                let mut marble_builder = MarbleBuilder::default();

                marble_builder.with_seed(*seed);
                marble_builder.with_frequency(*frequency);

                Ok(Arc::new(marble_builder.build()))
            },
            Self::Noise {
                seed,
                frequency,
                lacunarity,
                octaves,
                persistence,
            } => {
                let mut noise_builder = PerlinRidgedNoiseBuilder::default();

                noise_builder.with_seed(*seed);
                noise_builder.with_frequency(*frequency);
                noise_builder.with_lacunarity(*lacunarity);
                noise_builder.with_octaves(*octaves);
                noise_builder.with_persistence(*persistence);

                Ok(Arc::new(noise_builder.build()))
            },
            Self::SolidColor { color } => {
                Ok(Arc::new(SolidColor::new(*color)))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MaterialConfig {
    Dielectric {
        refraction_index: f64,
    },
    DiffuseLight {
        intensity: f64,
        texture: usize,
    },
    Lambertian {
        texture: usize,
    },
    Metal {
        fuzz: f64,
        texture: usize,
    },
}

impl MaterialConfig {
    pub fn try_make_material(
        &self,
        textures: &[Arc<dyn Texture + Send + Sync>],
    ) -> Result<Arc<dyn Material + Send + Sync>> {
        match self {
            Self::Dielectric { refraction_index } => {
                Ok(Arc::new(Dielectric::new(*refraction_index)))
            },
            Self::DiffuseLight { intensity, texture } => {
                let mut diffuse_light_builder = DiffuseLightBuilder::default();

                diffuse_light_builder.with_intensity(*intensity);
                diffuse_light_builder.with_texture(textures
                    .get(*texture)
                    .ok_or(anyhow!("invalid texture index"))?
                    .clone()
                );

                Ok(Arc::new(diffuse_light_builder.build()))
            },
            Self::Lambertian { texture } => {
                Ok(Arc::new(Lambertian::with_texture(textures
                    .get(*texture)
                    .ok_or(anyhow!("invalid texture index"))?
                    .clone()
                )))
            },
            Self::Metal { texture, fuzz } => {
                let mut metal_builder = MetalBuilder::default();

                metal_builder.with_texture(Some(
                    textures
                        .get(*texture)
                        .ok_or(anyhow!("invalid texture index"))?
                        .clone()
                ));
                metal_builder.with_fuzz(Some(*fuzz));
                Ok(Arc::new(metal_builder.build()))
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ObjectConfig {
    Plane {
        point: DVec3,
        u: DVec3,
        v: DVec3,
        material: usize,
    },
    Sphere {
        center: DVec3,
        radius: f64,
        material: usize,
    },
    Group {
        count: usize,
    },
    RotateY {
        angle: f64,
    },
    Translate {
        offset: DVec3,
    },
}

impl ObjectConfig {
    pub fn try_make_object(
        objects: &mut VecDeque<ObjectConfig>,
        materials: &[Arc<dyn Material + Send + Sync>],
    ) -> Result<Option<Arc<dyn Hitable + Send + Sync>>> {
        match objects.pop_front() {
            Some(Self::Plane { point: top_left, u, v, material }) => {
                let mut quad_builder = PlaneBuilder::default();
                let material = materials
                    .get(material)
                    .ok_or(anyhow!("invalid material index"))?
                    .clone();

                quad_builder.with_point(top_left);
                quad_builder.with_u(u);
                quad_builder.with_v(v);
                quad_builder.with_material(material);

                Ok(Some(Arc::new(quad_builder.build())))
            },
            Some(Self::Sphere { center, radius, material }) => {
                let mut sphere_builder = SphereBuilder::default();
                let material = materials
                    .get(material)
                    .ok_or(anyhow!("invalid material index"))?
                    .clone();

                sphere_builder.with_center(center);
                sphere_builder.with_radius(radius);
                sphere_builder.with_material(material);

                Ok(Some(Arc::new(sphere_builder.build())))
            },
            Some(Self::Group { count }) => {
                let mut group = Vec::new();

                for _ in 0..count {
                    if let Some(object) = Self::try_make_object(objects, materials)? {
                        group.push(object);
                    } else {
                        return Err(anyhow!("Object creation failed"));
                    }
                }

                Ok(Some(Arc::new(BVH::from(group.as_mut_slice()))))
            },
            Some(Self::RotateY { angle }) => {
                if let Some(object) = Self::try_make_object(objects, materials)? {
                    Ok(Some(Arc::new(RotateY::new(object, angle))))
                } else {
                    Err(anyhow!("Object creation failed"))
                }
            },
            Some(Self::Translate { offset }) => {
                if let Some(object) = Self::try_make_object(objects, materials)? {
                    Ok(Some(Arc::new(Translate::new(object, offset))))
                } else {
                    Err(anyhow!("Object creation failed"))
                }
            },
            None => Ok(None)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub textures: VecDeque<TextureConfig>,
    pub materials: VecDeque<MaterialConfig>,
    pub objects: VecDeque<ObjectConfig>,
}
