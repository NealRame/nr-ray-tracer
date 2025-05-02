use glam::DVec3;

use itertools::Itertools;

pub struct Image {
    aspect_ratio: f64,
    height: usize,
    width: usize,
    pixels: Vec<DVec3>,
}

impl Image {
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {
        let aspect_ratio = (width as f64)/(height as f64);
        let pixels = vec![DVec3::ZERO; width*height];

        Self {
            aspect_ratio,
            width,
            height,
            pixels,
        }
    }

    pub fn new_with_width_and_aspect_ratio(
        width: usize,
        aspect_ratio: f64,
    ) -> Self {
        let height = (((width as f64)/aspect_ratio) as usize).max(1);
        Image::new(width, height)
    }

    pub fn new_with_height_and_aspect_ratio(
        height: usize,
        aspect_ratio: f64,
    ) -> Self {
        let width = (((height as f64)*aspect_ratio) as usize).max(1);
        Image::new(width, height)
    }
}

impl Image {
    pub fn get_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
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
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.pixels.get(y*self.width + x)
        } else {
            None
        }
    }

    pub fn get_pixel_at_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> Option<&mut DVec3> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.pixels.get_mut(y*self.width + x)
        } else {
            None
        }
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ((usize, usize), &DVec3)> {
        self.pixels.iter().enumerate().map(|(index, pixel)| {
            let x = index%self.width;
            let y = index/self.width;

            ((x, y), pixel)
        })
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = ((usize, usize), &mut DVec3)> {
        self.pixels.iter_mut().enumerate().map(|(index, pixel)| {
            let x = index%self.width;
            let y = index/self.width;

            ((x, y), pixel)
        })
    }

    pub fn map<F>(
        &mut self,
        mut f: F
    ) -> &mut Self where F: FnMut(usize, usize) -> DVec3  {
        Itertools::cartesian_product(0..self.height, 0..self.width)
            .for_each(|(y, x)| {
                self.pixels[y*self.width + x] = f(x, y);
            });
        self
    }
}
