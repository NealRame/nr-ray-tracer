use std::f64::INFINITY;
use std::usize;

use glam::{
    DVec2,
    DVec3,
};

use image::Rgb32FImage;

use indicatif::ProgressBar;

use rand::rngs::ThreadRng;

use crate::hitable::Hitable;
use crate::image::ImageSize;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::*;

pub struct Camera {
    eye: DVec3,

    image_size: ImageSize,

    max_depth: usize,

    sample_per_pixels: Option<usize>,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

#[derive(Clone, Copy, Default)]
pub struct CameraBuilder {
    eye: Option<DVec3>,
    focal_length: Option<f64>,

    image_size: ImageSize,

    max_depth: Option<usize>,
    sample_per_pixels: Option<usize>,
}

impl CameraBuilder {
    pub fn new(image_size: ImageSize) -> Self {
        Self {
            image_size,
            ..Self::default()
        }
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
    ) -> Camera {
        let image_size = self.image_size;

        let eye = self.eye.unwrap_or_default();
        let focal_length = self.focal_length.unwrap_or(1.0);
        let max_depth = self.max_depth.unwrap_or(10);

        let sample_per_pixels = self.sample_per_pixels;

        let viewport_height = 2.0;
        let viewport_width = image_size.get_aspect_ratio()*viewport_height;

        let viewport_u =  DVec3::X*viewport_width;
        let viewport_v = -DVec3::Y*viewport_height;

        let viewport_pixel_delta_u = viewport_u/(image_size.width as f64);
        let viewport_pixel_delta_v = viewport_v/(image_size.height as f64);

        let viewport_top_left =
                eye - DVec3::Z*focal_length
                    - viewport_u/2.
                    - viewport_v/2.
                    + (viewport_pixel_delta_u + viewport_pixel_delta_v)/2.
                ;

        Camera {
            eye,
            image_size,
            max_depth,
            sample_per_pixels,
            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl Camera {
    fn ray_color(
        &self,
        rng: &mut ThreadRng,
        ray: &Ray,
        hitable: &impl Hitable,
        depth: usize,
    ) -> DVec3 {
        if depth >= self.max_depth {
            return DVec3::ZERO;
        }

        match hitable.hit(ray, Interval::new(0.001, INFINITY)) {
            Some(hit_record) => {
                // let reflected_direction = random_on_hemisphere(rng, hit_record.normal);
                let reflected_direction = hit_record.normal + random_in_unit_sphere(rng);
                let reflected_ray = Ray::new(hit_record.point, reflected_direction);

                self.ray_color(rng, &reflected_ray, hitable, depth + 1)/2.0
            },
            _ => {
                let d = ray.get_direction().normalize();
                let a = (d.y + 1.)/2.;

                (1. - a)*DVec3::ONE + a*DVec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn render_with_anti_aliasing(
        &self,
        hitable: &impl Hitable,
        progress: Option<&ProgressBar>,
    ) -> Rgb32FImage {
        let mut rng = rand::rng();
        let sample_per_pixels = self.sample_per_pixels.unwrap();
        let width = self.image_size.width as u32;
        let height = self.image_size.height as u32;

        Rgb32FImage::from_fn(width, height, |x, y| {
            let s = (0..sample_per_pixels).map(|_| {
                let offset = DVec2::from_rng_ranged(&mut rng, -0.5..0.5);

                let point =
                    self.viewport_top_left
                        + (x as f64 + offset.x)*self.viewport_pixel_delta_u
                        + (y as f64 + offset.y)*self.viewport_pixel_delta_v
                    ;

                let direction = point - self.eye;
                let ray = Ray::new(self.eye, direction);

                self.ray_color(&mut rng, &ray, hitable, 0)
            }).sum::<DVec3>();

            let color = s/(sample_per_pixels as f64);

            if let Some(bar) = progress {
                bar.inc(1);
            }

            image::Rgb(color.as_vec3().to_array())
        })
    }

    fn render_without_anti_aliasing(
        &self,
        hitable: &impl Hitable,
        progress: Option<&ProgressBar>,
    ) -> Rgb32FImage {
        let mut rng = rand::rng();
        let width = self.image_size.width as u32;
        let height = self.image_size.height as u32;

        Rgb32FImage::from_fn(width, height, |x, y| {
            let pixel =
                self.viewport_top_left
                    + (x as f64)*self.viewport_pixel_delta_u
                    + (y as f64)*self.viewport_pixel_delta_v
                ;

            let direction = pixel - self.eye;
            let ray = Ray::new(self.eye, direction);
            let color = self.ray_color(&mut rng, &ray, hitable, 0);

            if let Some(bar) = progress {
                bar.inc(1);
            }

            image::Rgb(color.as_vec3().to_array())
        })
    }

    pub fn render(
        &self,
        hitable: &impl Hitable,
        progress: Option<&ProgressBar>,
    ) -> Rgb32FImage {
        let progress = progress.map(|bar| {
            bar.set_position(0);
            bar.set_length(self.image_size.get_pixel_count() as u64);
            bar
        });

        match self.sample_per_pixels {
            Some(sample_per_pixels) if sample_per_pixels > 1 => {
                self.render_with_anti_aliasing(hitable, progress)
            },
            _ => {
                self.render_without_anti_aliasing(hitable, progress)
            }
        }
    }
}
