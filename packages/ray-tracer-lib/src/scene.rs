use std::collections::HashMap;
use std::sync::Arc;

use glam::DVec3;

use image::Rgb32FImage;

use serde::{
    Deserialize,
    Serialize,
};

use crate::camera::*;
use crate::image::ImageSize;
use crate::materials::*;
use crate::objects::*;

#[derive(Deserialize, Serialize)]
pub struct CameraDefinition {
    field_of_view: f64,
    look_at: DVec3,
    look_from: DVec3,
    view_up: DVec3,
    resolution: ImageSize,
}

#[derive(Deserialize, Serialize)]
pub enum MaterialDefinition {
    Dielectric{ refraction_index: Option<f64>, },
    Lambertian{ albedo: Option<DVec3>, },
    Metal{ albedo: Option<DVec3>, fuzz: Option<f64>, },
}

impl From<&MaterialDefinition> for Arc<dyn Material + Send + Sync> {
    fn from(value: &MaterialDefinition) -> Self {
        match value {
            MaterialDefinition::Dielectric{refraction_index} => {
                let mut dielectric = Dielectric::default();
                if let Some(refraction_index) = refraction_index.clone() {
                    dielectric.refraction_index = refraction_index;
                }

                Arc::new(dielectric) as Arc<dyn Material + Send + Sync>
            },
            MaterialDefinition::Lambertian { albedo } => {
                let mut lambertian = Lambertian::default();

                if let Some(albedo) = albedo.clone() {
                    lambertian.albedo = albedo;
                }

                Arc::new(lambertian) as Arc<dyn Material + Send + Sync>
            },
            MaterialDefinition::Metal { albedo, fuzz } => {
                let mut metal = Metal::default();

                if let Some(albedo) = albedo.clone() {
                    metal.albedo = albedo;
                }

                if let Some(fuzz) = fuzz.clone() {
                    metal.fuzz = fuzz;
                }

                Arc::new(metal) as Arc<dyn Material + Send + Sync>
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum ObjectDefinition {
    Sphere{ center: DVec3, radius: f64, material: String, }
}

#[derive(Deserialize, Serialize)]
struct Config {
    camera: CameraDefinition,
    materials: HashMap<String, MaterialDefinition>,
    objects: Vec<ObjectDefinition>,
}

#[derive(Clone)]
pub enum Error {
    UndefinedMaterial(String),
}

pub struct Scene {
    // materials: HashMap<String, Arc<dyn Material + Send + Sync>>,
    camera: Camera,
    objects: HitableList,
}

impl TryFrom<Config> for Scene {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let materials =
            config.materials
                .iter()
                .map(|(id, def)| (id.clone(), def.into()))
                .collect::<HashMap<String, Arc<dyn Material + Send + Sync>>>()
            ;

        let camera =
            CameraBuilder::new(config.camera.resolution)
                .with_look_at(config.camera.look_at)
                .with_look_from(config.camera.look_from)
                .with_view_up(config.camera.view_up)
                .build()
            ;

        let mut objects = HitableList::new();

        for object in config.objects {
            match object {
                ObjectDefinition::Sphere { center, radius, material } => {
                    materials.get(&material)
                        .ok_or(Error::UndefinedMaterial(material))
                        .and_then(|material| {
                            objects.add(Box::new(Sphere::new(
                                center,
                                radius,
                                material.clone(),
                            )));
                            Ok(())
                        })?;
                }
            }
        }

        // Ok(Self { materials, camera, objects })
        Ok(Self { camera, objects })
    }
}

impl Scene {
    pub fn render<P>(
        &self,
        progress: Option<P>,
    ) -> Rgb32FImage where P: Fn() + Sync {
        self.camera.render(&self.objects, progress)
    }
}
