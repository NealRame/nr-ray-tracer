use std::collections::HashMap;
use std::fs;
use std::ffi::OsStr;
use std::path::{
    Path,
    PathBuf,
};
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
        even: Option<Box<str>>,
        odd: Option<Box<str>>,
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
        textures: &HashMap<Box<str>, Arc<dyn Texture + Send + Sync>>,
    ) -> Result<Arc<dyn Texture + Send + Sync>> {
        match self {
            Self::Checker {
                scale,
                even,
                odd,
            } => {
                let mut checker_builder = CheckerBuilder::default();

                if let Some(even) = even {
                    checker_builder.with_even_texture(Some(
                        textures
                            .get(even)
                            .ok_or(anyhow!("invalid texture index"))?
                            .clone()
                    ));
                }

                if let Some(odd) = odd {
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
        texture: Box<str>,
    },
    Lambertian {
        texture: Box<str>,
    },
    Metal {
        fuzz: f64,
        texture: Box<str>,
    },
}

impl MaterialConfig {
    pub fn try_make_material(
        &self,
        textures: &HashMap<Box<str>, Arc<dyn Texture + Send + Sync>>,
    ) -> Result<Arc<dyn Material + Send + Sync>> {
        match self {
            Self::Dielectric {refraction_index } => {
                Ok(Arc::new(Dielectric::new(*refraction_index)))
            },
            Self::DiffuseLight { intensity, texture } => {
                let mut diffuse_light_builder = DiffuseLightBuilder::default();

                diffuse_light_builder.with_intensity(*intensity);
                diffuse_light_builder.with_texture(textures
                    .get(texture)
                    .ok_or(anyhow!("invalid texture index"))?
                    .clone()
                );

                Ok(Arc::new(diffuse_light_builder.build()))
            },
            Self::Lambertian { texture } => {
                let lambertian = Lambertian::with_texture(textures
                    .get(texture)
                    .ok_or(anyhow!("invalid texture index"))?
                    .clone()
                );

                Ok(Arc::new(lambertian))
            },
            Self::Metal { texture, fuzz } => {
                let mut metal_builder = MetalBuilder::default();

                metal_builder.with_texture(Some(
                    textures
                        .get(texture)
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
    Quad {
        point: DVec3,
        u: DVec3,
        v: DVec3,
        material: Box<str>,
    },
    Triangle {
        point: DVec3,
        u: DVec3,
        v: DVec3,
        material: Box<str>,
    },
    Sphere {
        center: DVec3,
        radius: f64,
        material: Box<str>,
    },
    Group {
        objects: Vec<ObjectConfig>,
    },
    RotateY {
        angle: f64,
        object: Box<ObjectConfig>,
    },
    Scale {
        factor: f64,
        object: Box<ObjectConfig>,
    },
    Translate {
        offset: DVec3,
        object: Box<ObjectConfig>,
    },
    Ref {
        id: Box<str>,
    },
}

impl ObjectConfig {
    pub fn try_make_object(
        &self,
        instances: &HashMap<Box<str>, Arc<dyn Hitable + Send + Sync>>,
        materials: &HashMap<Box<str>, Arc<dyn Material + Send + Sync>>,
    ) -> Result<Arc<dyn Hitable + Send + Sync>> {
        match self {
            Self::Quad { point, u, v, material } => {
                let material = materials
                    .get(material)
                    .ok_or(anyhow!("invalid material index"))?
                    .clone();

                let mut plane_builder = PlaneBuilder::default();

                plane_builder.with_point(*point);
                plane_builder.with_u(*u);
                plane_builder.with_v(*v);
                plane_builder.with_shape(Shape::Quad);
                plane_builder.with_material(material);

                Ok(Arc::new(plane_builder.build()))
            },
            Self::Triangle { point, u, v, material } => {
                let material = materials
                    .get(material)
                    .ok_or(anyhow!("invalid material index"))?
                    .clone();

                let mut plane_builder = PlaneBuilder::default();

                plane_builder.with_point(*point);
                plane_builder.with_u(*u);
                plane_builder.with_v(*v);
                plane_builder.with_shape(Shape::Triangle);
                plane_builder.with_material(material);

                Ok(Arc::new(plane_builder.build()))
            },
            Self::Sphere { center, radius, material } => {
                let material = materials
                    .get(material)
                    .ok_or(anyhow!("invalid material index"))?
                    .clone();

                let mut sphere_builder = SphereBuilder::default();

                sphere_builder.with_center(*center);
                sphere_builder.with_radius(*radius);
                sphere_builder.with_material(material);

                Ok(Arc::new(sphere_builder.build()))
            },
            Self::Group { objects } => {
                let mut group = Vec::new();

                for object_config in objects {
                    let object = object_config.try_make_object(instances, materials)?;

                    group.push(object);
                }

                Ok(Arc::new(BVH::from(group.as_mut_slice())))
            },
            Self::RotateY { object, angle } => {
                let object = object.try_make_object(instances, materials)?;

                Ok(Arc::new(RotateY::new(object, *angle)))
            },
            Self::Scale { object, factor } => {
                let object = object.try_make_object(instances, materials)?;

                Ok(Arc::new(Scale::new(object, *factor)))
            },
            Self::Translate { object, offset } => {
                let object = object.try_make_object(instances, materials)?;

                Ok(Arc::new(Translate::new(object, *offset)))
            },
            Self::Ref { id } => {
                let object = instances
                    .get(id)
                    .ok_or(anyhow!("invalid object id"))?
                    .clone();

                Ok(object)
            },
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub textures: HashMap<Box<str>, TextureConfig>,
    pub materials: HashMap<Box<str>, MaterialConfig>,
    pub instances: HashMap<Box<str>, ObjectConfig>,
    pub scene: Vec<ObjectConfig>,
}

type TextureMap = HashMap<Box<str>, Arc<dyn Texture + Send + Sync>>;
type MaterialMap = HashMap<Box<str>, Arc<dyn Material + Send + Sync>>;
type InstanceMap = HashMap<Box<str>, Arc<dyn Hitable + Send + Sync>>;

impl SceneConfig {
    pub fn try_build(self) -> Result<Scene> {
        let mut textures = TextureMap::new();
        for (texture_id, texture_config) in self.textures {
            let texture = texture_config.try_make_texture(&textures)?;
            textures.insert(texture_id.clone(), texture);
        }

        let mut materials = MaterialMap::new();
        for (material_id, material_config) in self.materials {
            let material = material_config.try_make_material(&textures)?;
            materials.insert(material_id.clone(), material);
        }

        let mut instances = InstanceMap::new();
        for (instance_id, instance_config) in self.instances {
            let object = instance_config.try_make_object(
                &instances,
                &materials,
            )?;
            instances.insert(instance_id.clone(), object);
        }

        let mut objects = Vec::new();
        for object_config in self.scene {
            let object = object_config.try_make_object(
                &instances,
                &materials,
            )?;
            objects.push(object);
        }

        let mut camera_builder = CameraBuilder::default();

        self.camera.try_update(&mut camera_builder)?;

        let camera = camera_builder.build();

        Ok(Scene {
            camera,
            objects: BVH::from(objects.as_mut_slice()),
        })
    }

    pub fn try_load_scene<P: AsRef<Path>>(path: P) -> Result<Self> {
        let ext = path.as_ref().extension().and_then(OsStr::to_str);
        let scene_config = match ext {
            Some("json") => {
                let s = fs::read_to_string(path.as_ref())?;
                serde_json::from_str::<SceneConfig>(&s)?
            },
            Some("toml") => {
                let s = fs::read_to_string(path.as_ref())?;
                toml::from_str::<SceneConfig>(&s)?
            },
            _ => {
                return Err(anyhow!("invalid scene file format!"));
            }
        };

        Ok(scene_config)
    }
}
