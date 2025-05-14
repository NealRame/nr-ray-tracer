use image::Rgb32FImage;

use serde::{
    Deserialize,
    Serialize,
};

use crate::camera::*;
use crate::objects::*;

#[derive(Deserialize, Serialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub objects: Vec<Object>,
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn render<P>(
        &self,
        progress: Option<P>,
    ) -> Rgb32FImage where P: Fn() + Sync {
        self.camera.render(&self.objects, progress)
    }
}

impl From<SceneConfig> for Scene {
    fn from(config: SceneConfig) -> Self {
        Self {
            camera: config.camera.build(),
            objects: config.objects,
        }
    }
}
