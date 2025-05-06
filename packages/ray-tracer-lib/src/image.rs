use glam::DVec3;

use itertools::Itertools;

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

#[derive(Default)]
pub struct Image {
    size: ImageSize,
    pixels: Vec<DVec3>,
}

impl Image {
    pub fn new(size: ImageSize) -> Self {
        let pixels = vec![DVec3::ZERO; size.get_pixel_count()];

        Self {
            size,
            pixels,
        }
    }
}

impl Image {
    pub fn get_aspect_ratio(&self) -> f64 {
        self.size.get_aspect_ratio()
    }

    pub fn get_height(&self) -> usize {
        self.size.height
    }

    pub fn get_width(&self) -> usize {
        self.size.width
    }

    pub fn get_pixel_count(&self) -> usize {
        self.size.get_pixel_count()
    }

    pub fn get_pixels(&self) -> &[DVec3] {
        &self.pixels[..]
    }

    pub fn get_pixels_mut(&mut self) -> &mut [DVec3] {
        &mut self.pixels[..]
    }

    pub fn get_pixel_at(
        &self,
        x: usize,
        y: usize,
    ) -> Option<&DVec3> {
        if (0..self.size.width).contains(&x)
        && (0..self.size.height).contains(&y) {
            self.pixels.get(y*self.size.width + x)
        } else {
            None
        }
    }

    pub fn get_pixel_at_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> Option<&mut DVec3> {
        if (0..self.size.width).contains(&x)
        && (0..self.size.height).contains(&y) {
            self.pixels.get_mut(y*self.size.width + x)
        } else {
            None
        }
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ((usize, usize), &DVec3)> {
        self.pixels.iter().enumerate().map(|(index, pixel)| {
            let x = index%self.size.width;
            let y = index/self.size.width;

            ((x, y), pixel)
        })
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = ((usize, usize), &mut DVec3)> {
        self.pixels.iter_mut().enumerate().map(|(index, pixel)| {
            let x = index%self.size.width;
            let y = index/self.size.width;

            ((x, y), pixel)
        })
    }

    pub fn map<F>(
        &mut self,
        mut f: F,
    ) -> &mut Self where F: FnMut(usize, usize) -> DVec3 {
        Itertools::cartesian_product(0..self.size.height, 0..self.size.width)
            .for_each(|(y, x)| {
                self.pixels[y*self.size.width + x] = f(x, y);
            });
        self
    }
}
