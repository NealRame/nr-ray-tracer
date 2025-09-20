use std::f64::consts::PI;
use std::f64::INFINITY;
use std::usize;

use glam::{
    DVec2,
    DVec3,
};

use image::Rgb32FImage;

use rand::Rng;

use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;

use rayon::iter::{
    IntoParallelIterator,
    ParallelIterator,
};

use crate::hitable::Hitable;
use crate::image::ImageSize;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::*;


#[derive(Clone, Copy, Debug)]
pub struct CameraBuilder {
    image_size: ImageSize,
    background_color: DVec3,
    look_from: DVec3,
    look_at: DVec3,
    view_up: DVec3,
    defocus_angle: f64,
    focus_dist: f64,
    field_of_view: f64,
    ray_max_bounces: usize,
    samples_per_pixel: usize,
}

impl CameraBuilder {
    pub fn with_image_size(&mut self, value: ImageSize) -> &mut Self {
        self.image_size = value;
        self
    }

    pub fn with_background_color(&mut self, value: DVec3) -> &mut Self {
        self.background_color = value;
        self
    }

    pub fn with_look_at(&mut self, value: DVec3) -> &mut Self {
        self.look_at = value;
        self
    }

    pub fn with_look_from(&mut self, value: DVec3) -> &mut Self {
        self.look_from = value;
        self
    }

    pub fn with_view_up(&mut self, value: DVec3) -> &mut Self {
        self.view_up = value;
        self
    }

    pub fn with_defocus_angle(&mut self, value: f64) -> &mut Self {
        self.defocus_angle = value;
        self
    }

    pub fn with_focus_dist(&mut self, value: f64) -> &mut Self {
        self.focus_dist = value;
        self
    }

    pub fn with_field_of_view(&mut self, value: f64) -> &mut Self {
        self.field_of_view = value;
        self
    }

    pub fn with_ray_max_bounces(&mut self, value: usize) -> &mut Self {
        self.ray_max_bounces = value;
        self
    }

    pub fn with_samples_per_pixel(&mut self, value: usize) -> &mut Self {
        self.samples_per_pixel = value;
        self
    }

    pub fn build(self) -> Camera {
        let image_size = self.image_size;

        let background_color = self.background_color;

        let look_at = self.look_at;
        let look_from = self.look_from;
        let view_up = self.view_up;

        let ray_max_bounces = self.ray_max_bounces;
        let samples_per_pixel = self.samples_per_pixel.max(1);

        let defocus_angle = self.defocus_angle.clamp(0., PI);
        let focus_dist = self.focus_dist;

        let field_of_view = self.field_of_view;
        let h = (field_of_view/2.).tan();

        let viewport_height = focus_dist*h*2.0;
        let viewport_width = viewport_height*image_size.get_aspect_ratio();

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u).normalize();

        let viewport_u = u*viewport_width;
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

            background_color,

            look_from,
            // look_at,
            // view_up,
            // defocus_angle,
            // field_of_view,
            // focus_dist,

            ray_max_bounces,
            samples_per_pixel,

            defocus_disk_u,
            defocus_disk_v,

            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl CameraBuilder {
    pub const DEFAULT_IMAGE_WIDTH: usize = 1200;
    pub const DEFAULT_IMAGE_HEIGHT: usize = 800;

    pub const DEFAULT_BACKGROUND_COLOR: DVec3 = DVec3::ZERO;

    pub const DEFAULT_LOOK_AT: DVec3 = DVec3::ZERO;
    pub const DEFAULT_LOOK_FROM: DVec3 = DVec3::ONE;
    pub const DEFAULT_VIEW_UP: DVec3 = DVec3::Y;

    pub const DEFAULT_DEFOCUS_ANGLE: f64 = 0.;
    pub const DEFAULT_FIELD_OF_VIEW: f64 = PI/2.;
    pub const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
    pub const DEFAULT_FOCUS_DISTANCE: f64 = 1.0;

    pub const DEFAULT_RAY_MAX_BOUNCES: usize = 10;
    pub const DEFAULT_SAMPLES_PER_PIXEL: usize = 10;
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_size: ImageSize {
                height: Self::DEFAULT_IMAGE_HEIGHT,
                width: Self::DEFAULT_IMAGE_WIDTH,
            },

            background_color: Self::DEFAULT_BACKGROUND_COLOR,

            look_at: Self::DEFAULT_LOOK_AT,
            look_from: Self::DEFAULT_LOOK_FROM,
            view_up: Self::DEFAULT_VIEW_UP,

            defocus_angle: Self::DEFAULT_DEFOCUS_ANGLE,
            focus_dist: Self::DEFAULT_FOCUS_DISTANCE,
            field_of_view: Self::DEFAULT_FIELD_OF_VIEW,

            ray_max_bounces: Self::DEFAULT_RAY_MAX_BOUNCES,
            samples_per_pixel: Self::DEFAULT_SAMPLES_PER_PIXEL,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    image_size: ImageSize,

    background_color: DVec3,

    look_from: DVec3,
    // look_at: DVec3,
    // view_up: DVec3,
    // defocus_angle: f64,
    // focus_dist: f64,
    // field_of_view: f64,

    ray_max_bounces: usize,
    samples_per_pixel: usize,

    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

impl Camera {
    pub fn get_image_size(&self) -> ImageSize {
        self.image_size
    }
}

impl Camera {
    fn defocus_disk_sample(
        &self,
        rng: &mut impl Rng,
    ) -> DVec3 {
        let p = random_in_unit_disk(rng);
        self.look_from + p.x*self.defocus_disk_u + p.y*self.defocus_disk_v
    }

    fn get_ray(
        &self,
        x: u32,
        y: u32,
        rng: &mut impl Rng,
    ) -> Ray {
        let offset = if self.samples_per_pixel > 1 {
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
        let time = rng.random_range(0.0..1.0);

        Ray::new_at_time(origin, direction, time)
    }

    fn get_ray_color(
        &self,
        ray: &Ray,
        ray_bounce: usize,
        hitable: &impl Hitable,
        rng: &mut impl Rng,
    ) -> DVec3 {
        if ray_bounce >= self.ray_max_bounces {
            return DVec3::ZERO;
        }

        match hitable.hit(ray, Interval::new(0.001, INFINITY)).as_ref() {
            Some(hit_record) => {
                let material = hit_record.material.clone();
                let emitted = material.emit(ray, hit_record);

                material.scatter(ray, hit_record, rng)
                    .map(|(mut scattered_ray, color)| {
                        scattered_ray.bounce();

                        emitted + color*self.get_ray_color(
                            &scattered_ray,
                            ray_bounce + 1,
                            hitable,
                            rng
                        )
                    })
                    .unwrap_or(emitted)
            },
            _ => self.background_color,
        }
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
        let sample_per_pixel = self.samples_per_pixel;
        let width = self.image_size.width as u32;
        let height = self.image_size.height as u32;

        let pixels = (0..width*height)
            .into_par_iter()
            .map(|n| {
                let mut rng = ChaCha8Rng::seed_from_u64(0);

                rng.set_stream(n as u64);

                let x = n%width;
                let y = n/width;

                let s = (0..sample_per_pixel).map(|_| {
                    let ray = self.get_ray(x, y, &mut rng);

                    self.get_ray_color(&ray, 0, hitable, &mut rng)
                }).sum::<DVec3>();

                let color = s/(sample_per_pixel as f64);

                if let Some(progress) = progress.as_ref() {
                    progress();
                }

                color.as_vec3().to_array()
            })
            .flatten()
            .collect::<Vec<_>>();

        Rgb32FImage::from_vec(width, height, pixels).unwrap()
    }
}
