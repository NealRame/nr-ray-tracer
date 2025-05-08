use image::Rgb32FImage;

#[derive(Clone, Copy, Default)]
pub struct ImageSize {
    pub height: usize,
    pub width: usize,
}

impl ImageSize {
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {
        Self { width, height }
    }

    pub fn from_width_and_aspect_ratio(
        width: usize,
        aspect_ratio: f64,
    ) -> Self {
        let height = (((width as f64)/aspect_ratio) as usize).max(1);
        Self::new(width, height)
    }

    pub fn from_height_and_aspect_ratio(
        height: usize,
        aspect_ratio: f64,
    ) -> Self {
        let width = (((height as f64)*aspect_ratio) as usize).max(1);
        Self::new(width, height)
    }
}

impl ImageSize {
    pub fn get_aspect_ratio(&self) -> f64 {
        (self.width as f64)/(self.height as f64)
    }

    pub fn get_pixel_count(&self) -> usize {
        self.width*self.height
    }
}

pub fn gamma_correction(image: &mut Rgb32FImage, gamma: f32) {
    image.iter_mut().for_each(|p| {
        *p = p.powf(gamma)
    });
}
