use image::Rgb32FImage;

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use crate::camera::*;
use crate::objects::*;
use crate::materials::Material;
use crate::textures::Texture;

#[derive(Deserialize, Serialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub objects: Vec<Object>,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "SceneConfig")]
#[serde(into = "SceneConfig")]
pub struct Scene {
    pub camera: Camera,
    pub objects: BVH,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>,
}

impl Scene {
    pub fn render<P>(
        &self,
        progress: Option<P>,
    ) -> Rgb32FImage where P: Fn() + Sync {
        self.camera.render(self, &self.objects, progress)
    }
}

impl From<SceneConfig> for Scene {
    fn from(mut config: SceneConfig) -> Self {
        let camera = config.camera.build();
        let objects = BVH::from(config.objects.as_mut_slice());

        Self {
            camera,
            objects,
            materials: config.materials,
            textures: config.textures,
        }
    }
}

impl Into<SceneConfig> for Scene {
    fn into(self) -> SceneConfig {
        SceneConfig {
            camera: self.camera.into(),
            objects: self.objects.into(),
            materials: self.materials,
            textures: self.textures,
        }
    }
}
