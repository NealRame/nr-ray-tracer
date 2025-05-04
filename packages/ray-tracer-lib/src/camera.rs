use std::usize;

use glam::DVec3;

use rand::Rng;

use crate::hitable::HitableList;
use crate::image::Image;
use crate::ray::Ray;

pub struct Camera {
    eye: DVec3,

    image: Image,

    sample_per_pixels: Option<usize>,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

#[derive(Clone, Copy, Default)]
pub struct CameraBuilder {
    eye: Option<DVec3>,
    focal_length: Option<f64>,
    sample_per_pixels: Option<usize>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_focal_length(
        &mut self,
        focal_length: f64,
    ) -> &mut Self {
        self.focal_length.replace(focal_length);
        self
    }

    pub fn with_eye_at(
        &mut self,
        position: DVec3,
    ) -> &mut Self {
        self.eye.replace(position);
        self
    }

    pub fn with_sample_per_pixels(
        &mut self,
        count: usize,
    ) -> &mut Self {
        self.sample_per_pixels.replace(count);
        self
    }

    pub fn build(
        self,
        image: Image,
    ) -> Camera {
        let eye = self.eye.unwrap_or_default();
        let focal_length = self.focal_length.unwrap_or(1.0);

        let viewport_height = 2.0;
        let viewport_width = image.get_aspect_ratio()*viewport_height;

        let viewport_u =  DVec3::X*viewport_width;
        let viewport_v = -DVec3::Y*viewport_height;

        let viewport_pixel_delta_u = viewport_u/(image.get_width() as f64);
        let viewport_pixel_delta_v = viewport_v/(image.get_height() as f64);

        let viewport_top_left =
                eye - DVec3::Z*focal_length
                    - viewport_u/2.
                    - viewport_v/2.
                    + (viewport_pixel_delta_u + viewport_pixel_delta_v)/2.
                ;

        Camera {
            eye,

            image,

            sample_per_pixels: self.sample_per_pixels,

            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl Camera {
    pub fn render_with_anti_aliasing<F>(
        &mut self,
        world: &HitableList,
        mut f: F,
        sample_per_pixels: usize,
    ) -> &mut Self where F: FnMut(&Ray, &HitableList) -> DVec3 {
        let mut rng = rand::rng();

        self.image.map(|x, y| {
            let s = (0..sample_per_pixels).map(|_| {
                let (offset_x, offset_y) = (
                    rng.random_range(-0.5..0.5),
                    rng.random_range(-0.5..0.5),
                );

                let pixel =
                    self.viewport_top_left
                        + (x as f64 + offset_x)*self.viewport_pixel_delta_u
                        + (y as f64 + offset_y)*self.viewport_pixel_delta_v
                    ;

                let direction = pixel - self.eye;
                let ray = Ray::new(self.eye, direction);

                f(&ray, &world)
            }).sum::<DVec3>();

            s/(sample_per_pixels as f64)
        });
        self
    }

    pub fn render_without_anti_aliasing<F>(
        &mut self,
        world: &HitableList,
        mut f: F
    ) -> &mut Self where F: FnMut(&Ray, &HitableList) -> DVec3 {
        self.image.map(|x, y| {
            let pixel =
                self.viewport_top_left
                    + (x as f64)*self.viewport_pixel_delta_u
                    + (y as f64)*self.viewport_pixel_delta_v
                ;

            let direction = pixel - self.eye;
            let ray = Ray::new(self.eye, direction);

            f(&ray, &world)
        });
        self
    }

    pub fn render<F>(
        &mut self,
        world: &HitableList,
        f: F
    ) -> &mut Self where F: FnMut(&Ray, &HitableList) -> DVec3 {
        match self.sample_per_pixels {
            Some(sample_per_pixels) if sample_per_pixels > 1 => {
                self.render_with_anti_aliasing(world, f, sample_per_pixels)
            },
            _ => {
                self.render_without_anti_aliasing(world, f)
            }
        }
    }
}

impl Camera {
    pub fn take_image(self) -> Image {
        self.image
    }
}
