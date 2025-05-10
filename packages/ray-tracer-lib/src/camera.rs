use std::f64::consts::PI;
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
    image_size: ImageSize,

    max_depth: usize,
    sample_per_pixels: Option<usize>,

    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
    eye: DVec3,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

#[derive(Clone, Copy, Default)]
pub struct CameraBuilder {
    look_at: Option<DVec3>,
    look_from: Option<DVec3>,
    view_up: Option<DVec3>,

    defocus_angle: Option<f64>,
    focus_dist: Option<f64>,
    vertical_field_of_view: Option<f64>,

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

    pub fn with_vertical_field_of_view(
        &mut self,
        vertical_field_of_view: f64,
    ) -> &mut Self {
        self.vertical_field_of_view.replace(vertical_field_of_view);
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

        let eye = self.look_from.unwrap_or(DVec3::ZERO);
        let look_at = self.look_at.unwrap_or(-DVec3::Z);
        let vup = self.view_up.unwrap_or(DVec3::Y);

        let max_depth = self.max_depth.unwrap_or(10);
        let sample_per_pixels = self.sample_per_pixels;

        let defocus_angle: f64 = self.defocus_angle.unwrap_or(0.).clamp(0., PI);
        let focus_dist = self.focus_dist.unwrap_or(1.);

        let theta = self.vertical_field_of_view.unwrap_or(PI/2.0);
        let h = (theta/2.).tan();

        let viewport_height = focus_dist*h*2.0;
        let viewport_width = viewport_height*image_size.get_aspect_ratio();

        let w = (eye - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u).normalize();

        let viewport_u =  u*viewport_width;
        let viewport_v = -v*viewport_height;

        let viewport_pixel_delta_u = viewport_u/(image_size.width as f64);
        let viewport_pixel_delta_v = viewport_v/(image_size.height as f64);

        let viewport_top_left =
                eye - w*focus_dist
                    - viewport_u/2.
                    - viewport_v/2.
                    + (viewport_pixel_delta_u + viewport_pixel_delta_v)/2.
                ;

        let defocus_radius = (focus_dist/2.0)*defocus_angle.tan();
        let defocus_disk_u = u*defocus_radius;
        let defocus_disk_v = v*defocus_radius;

        Camera {
            image_size,

            max_depth,
            sample_per_pixels,

            defocus_disk_u,
            defocus_disk_v,
            eye,

            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl Camera {
    fn defocus_disk_sample(
        &self,
        rng: &mut ThreadRng,
    ) -> DVec3 {
        let p = random_in_unit_disk(rng);
        self.eye + p.x*self.defocus_disk_u + p.y*self.defocus_disk_v
    }

    fn get_ray(
        &self,
        x: u32,
        y: u32,
        rng: &mut ThreadRng,
    ) -> Ray {
        let offset = if self.sample_per_pixels.is_some() {
            DVec2::from_rng_ranged(rng, -0.5..0.5)
        } else {
            DVec2::ZERO
        };

        let point =
            self.viewport_top_left
                + (x as f64 + offset.x)*self.viewport_pixel_delta_u
                + (y as f64 + offset.y)*self.viewport_pixel_delta_v
            ;

        let direction = point - self.eye;
        let origin = self.defocus_disk_sample(rng);

        Ray::new(origin, direction)
    }

    fn get_ray_color(
        &self,
        ray: &Ray,
        hitable: &impl Hitable,
        depth: usize,
        rng: &mut ThreadRng,
    ) -> DVec3 {
        if depth >= self.max_depth {
            return DVec3::ZERO;
        }

        match hitable.hit(ray, Interval::new(0.001, INFINITY)).as_ref() {
            Some(hit_record) => {
                if let Some((scattered_ray, color)) = hit_record.material.scatter(ray, hit_record, rng) {
                    color*self.get_ray_color(&scattered_ray, hitable, depth + 1, rng)
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
                let ray = self.get_ray(x, y, &mut rng);

                self.get_ray_color(&ray, hitable, 0, &mut rng)
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
            let ray = self.get_ray(x, y, &mut rng);
            let color = self.get_ray_color(&ray, hitable, 0, &mut rng);

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
