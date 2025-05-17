use std::f64::consts::PI;
use std::f64::INFINITY;
use std::usize;

use glam::{
    DVec2,
    DVec3,
};

use image::Rgb32FImage;

use rand::rngs::ThreadRng;

use rayon::iter::{
    IntoParallelIterator,
    ParallelIterator,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::image::ImageSize;
use crate::interval::Interval;
use crate::objects::Hitable;
use crate::ray::Ray;
use crate::vector::*;

pub struct Camera {
    image_size: ImageSize,

    ray_max_bounce: usize,
    samples_per_pixel: Option<usize>,

    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,

    look_from: DVec3,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub struct CameraConfig {
    look_at: Option<DVec3>,
    look_from: Option<DVec3>,
    view_up: Option<DVec3>,

    #[serde(skip)]
    image_size: Option<ImageSize>,

    #[serde(skip)]
    defocus_angle: Option<f64>,

    #[serde(skip)]
    focus_dist: Option<f64>,

    #[serde(skip)]
    field_of_view: Option<f64>,

    #[serde(skip)]
    ray_max_bounce: Option<usize>,

    #[serde(skip)]
    samples_per_pixel: Option<usize>,
}

impl CameraConfig {
    pub fn with_image_size(
        &mut self,
        value: ImageSize,
    ) -> &mut Self {
        self.image_size.replace(value);
        self
    }

    pub fn with_look_at(
        &mut self,
        position: DVec3,
    ) -> &mut Self {
        self.look_at.replace(position);
        self
    }

    pub fn with_look_from(
        &mut self,
        position: DVec3,
    ) -> &mut Self {
        self.look_from.replace(position);
        self
    }

    pub fn with_view_up(
        &mut self,
        v: DVec3,
    ) -> &mut Self {
        self.view_up.replace(v);
        self
    }

    pub fn with_defocus_angle(
        &mut self,
        defocus_angle: f64,
    ) -> &mut Self {
        self.defocus_angle.replace(defocus_angle);
        self
    }

    pub fn with_focus_dist(
        &mut self,
        focus_dist: f64,
    ) -> &mut Self {
        self.focus_dist.replace(focus_dist);
        self
    }

    pub fn with_field_of_view(
        &mut self,
        vertical_field_of_view: f64,
    ) -> &mut Self {
        self.field_of_view.replace(vertical_field_of_view);
        self
    }

    pub fn with_ray_max_bounce(
        &mut self,
        max_depth: usize,
    ) -> &mut Self {
        self.ray_max_bounce.replace(max_depth);
        self
    }

    pub fn with_samples_per_pixel(
        &mut self,
        count: usize,
    ) -> &mut Self {
        self.samples_per_pixel.replace(count);
        self
    }

    pub fn build(
        &self,
    ) -> Camera {
        let image_size = self.image_size.unwrap_or_default();

        let look_from = self.look_from.unwrap_or(DVec3::ZERO);
        let look_at = self.look_at.unwrap_or(DVec3::NEG_Z);
        let view_up = self.view_up.unwrap_or(DVec3::Y);

        let ray_max_bounce = self.ray_max_bounce.unwrap_or(10);
        let samples_per_pixel = self.samples_per_pixel;

        let defocus_angle = self.defocus_angle.unwrap_or(0.0).clamp(0., PI);
        let focus_dist = self.focus_dist.unwrap_or(1.);

        let fov = self.field_of_view.unwrap_or(PI/2.0);
        let h = (fov/2.).tan();

        let viewport_height = focus_dist*h*2.0;
        let viewport_width = viewport_height*image_size.get_aspect_ratio();

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u).normalize();

        let viewport_u =  u*viewport_width;
        let viewport_v = -v*viewport_height;

        let viewport_pixel_delta_u = viewport_u/(image_size.width as f64);
        let viewport_pixel_delta_v = viewport_v/(image_size.height as f64);

        let viewport_top_left =
                look_from
                    - w*focus_dist
                    - viewport_u/2.0
                    - viewport_v/2.0
                    + (viewport_pixel_delta_u + viewport_pixel_delta_v)/2.0
                ;

        let defocus_radius = focus_dist*(defocus_angle/2.0).tan();
        let defocus_disk_u = u*defocus_radius;
        let defocus_disk_v = v*defocus_radius;

        Camera {
            image_size,

            ray_max_bounce,
            samples_per_pixel,

            defocus_disk_u,
            defocus_disk_v,
            look_from,

            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl Camera {
    pub fn get_image_size(&self) -> ImageSize {
        self.image_size
    }
}

impl Camera {
    fn defocus_disk_sample(
        &self,
        rng: &mut ThreadRng,
    ) -> DVec3 {
        let p = random_in_unit_disk(rng);
        self.look_from + p.x*self.defocus_disk_u + p.y*self.defocus_disk_v
    }

    fn get_ray(
        &self,
        x: u32,
        y: u32,
        rng: &mut ThreadRng,
    ) -> Ray {
        let offset = if self.samples_per_pixel.is_some() {
            DVec2::from_rng_ranged(rng, -0.5..=0.5)
        } else {
            DVec2::ZERO
        };

        let point =
            self.viewport_top_left
                + (x as f64 + offset.x)*self.viewport_pixel_delta_u
                + (y as f64 + offset.y)*self.viewport_pixel_delta_v
            ;

        let origin = self.defocus_disk_sample(rng);
        let direction = point - origin;

        Ray::new(origin, direction)
    }

    fn get_ray_color(
        &self,
        ray: &Ray,
        ray_bounce: usize,
        hitable: &impl Hitable,
        rng: &mut ThreadRng,
    ) -> DVec3 {
        if ray_bounce >= self.ray_max_bounce {
            return DVec3::ZERO;
        }

        match hitable.hit(ray, Interval::new(0.001, INFINITY)).as_ref() {
            Some(hit_record) => {
                if let Some((scattered_ray, color)) = hit_record.material.scatter(ray, hit_record, rng) {
                    color*self.get_ray_color(&scattered_ray, ray_bounce + 1, hitable, rng)
                } else {
                    DVec3::ZERO
                }
            },
            _ => {
                let d = ray.get_direction().normalize();
                let a = (d.y + 1.)/2.;

                (1. - a)*DVec3::ONE + a*DVec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn render_with_anti_aliasing<T, P>(
        &self,
        hitable: &T,
        progress: Option<&P>,
    ) -> Rgb32FImage
        where
            T: Hitable + Send + Sync,
            P: Fn() + Sync,
    {
        let sample_per_pixels = self.samples_per_pixel.unwrap();
        let width = self.image_size.width as u32;
        let height = self.image_size.height as u32;

        let pixels = (0..width*height)
            .into_par_iter()
            .map(|n| {
                let mut rng = rand::rng();

                let x = n%width;
                let y = n/width;

                let s = (0..sample_per_pixels).map(|_| {
                    let ray = self.get_ray(x, y, &mut rng);

                    self.get_ray_color(&ray, 0, hitable, &mut rng)
                }).sum::<DVec3>();

                let color = s/(sample_per_pixels as f64);

                if let Some(progress) = progress {
                    progress();
                }

                color.as_vec3().to_array()
            })
            .flatten()
            .collect::<Vec<_>>();

        Rgb32FImage::from_vec(width, height, pixels).unwrap()
    }

    fn render_without_anti_aliasing<T, P>(
        &self,
        hitable: &T,
        progress: Option<&P>,
    ) -> Rgb32FImage
        where
            T: Hitable + Send + Sync,
            P: Fn() + Sync,
    {
        let width = self.image_size.width as u32;
        let height = self.image_size.height as u32;

        let pixels = (0..width*height)
            .into_par_iter()
            .map(|n| {
                let mut rng = rand::rng();

                let x = n%width;
                let y = n/width;

                let ray = self.get_ray(x, y, &mut rng);
                let color = self.get_ray_color(&ray, 0, hitable, &mut rng);

                if let Some(progress) = progress {
                    progress();
                }

                color.as_vec3().to_array()
            })
            .flatten()
            .collect::<Vec<_>>();

        Rgb32FImage::from_vec(width, height, pixels).unwrap()
    }

    pub fn render<T, P>(
        &self,
        hitable: &T,
        progress: Option<P>,
    ) -> Rgb32FImage
        where
            T: Hitable + Send + Sync,
            P: Fn() + Sync,
    {
        let progress_ref = progress.as_ref();

        match self.samples_per_pixel {
            Some(sample_per_pixels) if sample_per_pixels > 1 => {
                self.render_with_anti_aliasing(hitable, progress_ref)
            },
            _ => {
                self.render_without_anti_aliasing(hitable, progress_ref)
            }
        }
    }
}
