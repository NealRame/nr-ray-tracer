use std::f64::INFINITY;
use std::usize;

use glam::{
    DVec2,
    DVec3,
};

use indicatif::ProgressBar;

use rand::rngs::ThreadRng;

use crate::hitable::Hitable;
use crate::image::Image;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::*;

pub struct Camera {
    eye: DVec3,

    image: Image,

    max_depth: usize,

    rng: ThreadRng,

    sample_per_pixels: Option<usize>,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

#[derive(Clone, Copy, Default)]
pub struct CameraBuilder {
    eye: Option<DVec3>,
    focal_length: Option<f64>,
    max_depth: Option<usize>,
    sample_per_pixels: Option<usize>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_eye_at(
        &mut self,
        position: DVec3,
    ) -> &mut Self {
        self.eye.replace(position);
        self
    }

    pub fn with_focal_length(
        &mut self,
        focal_length: f64,
    ) -> &mut Self {
        self.focal_length.replace(focal_length);
        self
    }

    pub fn with_max_depth(
        &mut self,
        max_depth: usize,
    ) -> &mut Self {
        self.max_depth.replace(max_depth);
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
        let max_depth = self.max_depth.unwrap_or(10);

        let rng = rand::rng();
        let sample_per_pixels = self.sample_per_pixels;

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
            max_depth,
            rng,
            sample_per_pixels,
            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl Camera {
    fn ray_color(
        &mut self,
        ray: &Ray,
        hitable: &impl Hitable,
        depth: usize,
    ) -> DVec3 {
        if depth >= self.max_depth {
            return DVec3::ZERO;
        }

        match hitable.hit(ray, Interval::new(0.001, INFINITY)) {
            Some(hit_record) => {
                let reflected_direction = random_on_hemisphere(&mut self.rng, hit_record.normal);
                let reflected_ray = Ray::new(hit_record.point, reflected_direction);

                self.ray_color(&reflected_ray, hitable, depth + 1)/2.0
            },
            _ => {
                let d = ray.get_direction().normalize();
                let a = (d.y + 1.)/2.;

                (1. - a)*DVec3::ONE + a*DVec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn render_with_anti_aliasing(
        &mut self,
        image: &mut Image,
        hitable: &impl Hitable,
        progress: Option<&ProgressBar>,
    ) {
        let sample_per_pixels = self.sample_per_pixels.unwrap();

        image.map(|x, y| {
            let s = (0..sample_per_pixels).map(|_| {
                let offset = DVec2::from_rng_ranged(&mut self.rng, -0.5..0.5);

                let pixel =
                    self.viewport_top_left
                        + (x as f64 + offset.x)*self.viewport_pixel_delta_u
                        + (y as f64 + offset.y)*self.viewport_pixel_delta_v
                    ;

                let direction = pixel - self.eye;
                let ray = Ray::new(self.eye, direction);

                self.ray_color(&ray, hitable, 0)
            }).sum::<DVec3>();

            if let Some(bar) = progress {
                bar.inc(1);
            }

            s/(sample_per_pixels as f64)
        });
    }

    fn render_without_anti_aliasing(
        &mut self,
        image: &mut Image,
        hitable: &impl Hitable,
        progress: Option<&ProgressBar>,
    ) {
        image.map(|x, y| {
            let pixel =
                self.viewport_top_left
                    + (x as f64)*self.viewport_pixel_delta_u
                    + (y as f64)*self.viewport_pixel_delta_v
                ;

            let direction = pixel - self.eye;
            let ray = Ray::new(self.eye, direction);
            let color = self.ray_color(&ray, hitable, 0);

            if let Some(bar) = progress {
                bar.inc(1);
            }

            color
        });
    }

    pub fn render(
        &mut self,
        hitable: &impl Hitable,
        progress: Option<ProgressBar>,
    ) -> &mut Self {
        let mut image = std::mem::take(&mut self.image);
        let progress = progress.map(|bar| {
            bar.set_length(image.get_pixel_count() as u64);
            bar
        });

        match self.sample_per_pixels {
            Some(sample_per_pixels) if sample_per_pixels > 1 => {
                self.render_with_anti_aliasing(&mut image, hitable, progress.as_ref());
            },
            _ => {
                self.render_without_anti_aliasing(&mut image, hitable, progress.as_ref());
            }
        }

        self.image = image;
        self
    }
}

impl Camera {
    pub fn take_image(self) -> Image {
        self.image
    }
}
