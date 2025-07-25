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

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use crate::hitable::Hitable;
use crate::image::ImageSize;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "CameraConfig")]
#[serde(into = "CameraConfig")]
pub struct Camera {
    image_size: ImageSize,

    look_at: DVec3,
    look_from: DVec3,
    view_up: DVec3,

    defocus_angle: f64,
    focus_dist: f64,
    field_of_view: f64,

    ray_max_bounce: usize,
    samples_per_pixel: usize,

    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename = "Camera")]
#[skip_serializing_none]
pub struct CameraConfig {
    image_size: Option<ImageSize>,

    look_at: Option<DVec3>,
    look_from: Option<DVec3>,
    view_up: Option<DVec3>,

    defocus_angle: Option<f64>,
    focus_dist: Option<f64>,
    field_of_view: Option<f64>,

    ray_max_bounce: Option<usize>,
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

        let look_at = self.look_at.unwrap_or(DVec3::NEG_Z);
        let look_from = self.look_from.unwrap_or(DVec3::ZERO);
        let view_up = self.view_up.unwrap_or(DVec3::Y);

        let ray_max_bounce = self.ray_max_bounce.unwrap_or(10);
        let samples_per_pixel = self.samples_per_pixel.unwrap_or(1).max(1);

        let defocus_angle = self.defocus_angle.unwrap_or(0.0).clamp(0., PI);
        let focus_dist = self.focus_dist.unwrap_or(1.);

        let field_of_view = self.field_of_view.unwrap_or(PI/2.0);
        let h = (field_of_view/2.).tan();

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

            look_at,
            look_from,
            view_up,

            defocus_angle,
            field_of_view,
            focus_dist,

            ray_max_bounce,
            samples_per_pixel,

            defocus_disk_u,
            defocus_disk_v,

            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl From<CameraConfig> for Camera {
    fn from(value: CameraConfig) -> Self {
        value.build()
    }
}

impl Into<CameraConfig> for Camera {
    fn into(self) -> CameraConfig {
        CameraConfig {
            image_size: Some(self.image_size),
            look_at: Some(self.look_at),
            look_from: Some(self.look_from),
            view_up: Some(self.view_up),
            defocus_angle: Some(self.defocus_angle),
            focus_dist: Some(self.focus_dist),
            field_of_view: Some(self.field_of_view),
            ray_max_bounce: Some(self.ray_max_bounce),
            samples_per_pixel: Some(self.samples_per_pixel),
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
        scene: &Scene,
        ray: &Ray,
        ray_bounce: usize,
        hitable: &impl Hitable,
        rng: &mut impl Rng,
    ) -> DVec3 {
        if ray_bounce >= self.ray_max_bounce {
            return DVec3::ZERO;
        }

        match hitable.hit(ray, Interval::new(0.001, INFINITY)).as_ref() {
            Some(hit_record) => {
                let material = scene.materials.get(hit_record.material).expect("material not found");

                material.scatter(scene, ray, hit_record, rng)
                    .as_ref()
                    .map(|(scattered_ray, color)| {
                        color*self.get_ray_color(
                            scene,
                            scattered_ray,
                            ray_bounce + 1,
                            hitable,
                            rng
                        )
                    })
                    .unwrap_or(DVec3::ZERO)
            },
            _ => {
                let d = ray.get_direction().normalize();
                let a = (d.y + 1.)/2.;

                (1. - a)*DVec3::ONE + a*DVec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn render<T, P>(
        &self,
        scene: &Scene,
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

                    self.get_ray_color(scene, &ray, 0, hitable, &mut rng)
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
