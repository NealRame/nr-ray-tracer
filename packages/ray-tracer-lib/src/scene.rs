use image::Rgb32FImage;

use crate::camera::*;
use crate::objects::*;

#[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    pub objects: BVH,
}

impl Scene {
    pub fn render<P>(
        &self,
        progress: Option<P>,
    ) -> Rgb32FImage where P: Fn() + Sync {
        self.camera.render(&self.objects, progress)
    }
}
