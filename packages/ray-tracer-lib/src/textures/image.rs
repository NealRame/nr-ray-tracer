use std::path::Path;

use glam::{
    DVec2,
    DVec3,
    Vec3,
};

use image::{
    ImageError,
    ImageReader,
    Rgb32FImage
};

use super::texture::Texture;

#[derive(Clone, Debug)]
pub struct Image {
    image: Rgb32FImage,
}

impl Image {
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let image = ImageReader::open(path.as_ref())?.decode()?.into_rgb32f();

        Ok(Self { image })
    }
}

impl Texture for Image {
    fn get_color(
        &self,
        uv_coord: DVec2,
        _: DVec3,
    ) -> DVec3 {
        let x = (uv_coord.x.clamp(0., 1.)*(self.image.width() as f64)) as u32;
        let y = ((1.0 - uv_coord.y.clamp(0., 1.))*(self.image.height() as f64)) as u32;

        Vec3::from_array(self.image.get_pixel(x, y).0).as_dvec3()
    }
}
