use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use itertools::{self, Itertools};

use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;

use nr_ray_tracer_lib::prelude::*;

use super::render;

#[derive(Clone)]
struct StepRange(f64, f64, f64);

impl Iterator for StepRange {
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let v = self.0;

            self.0 = self.0 + self.2;
            Some(v)
        } else {
            None
        }
    }
}

fn generate_objects() -> BVH {
    const SEED: u64 = 2;
    const GROUND_SPHERE_RADIUS: f64 = 1000.0;
    const SPHERE_RADIUS: f64 = 1.0;
    const MATERIAL_DISTRIBUTION: &[usize; 3] = &[1, 4, 8];

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let dist = WeightedIndex::new(MATERIAL_DISTRIBUTION).unwrap();

    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    let generate_dielectric = || {
        Arc::new(Dielectric::default())
    };

    let generate_lambertian = |rng: &mut ChaCha8Rng| {
        Arc::new(Lambertian::with_color(DVec3::from_rng(rng)))
    };

    let generate_metal = |rng: &mut ChaCha8Rng| {
        Arc::new(MetalBuilder::default()
            .with_color(DVec3::from_rng(rng))
            .with_fuzz(rng.random())
            .build()
        )
    };

    let mut checker_builder = CheckerBuilder::default();

    checker_builder.with_even_texture(Some(Arc::new(SolidColor::new(DVec3::new(0.2, 0.3, 0.1)))));
    checker_builder.with_odd_texture(Some(Arc::new(SolidColor::new(DVec3::new(0.9, 0.9, 0.9)))));
    checker_builder.with_scale(Some(128.));

    let checker = Arc::new(checker_builder.build());

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(GROUND_SPHERE_RADIUS*DVec3::NEG_Y)
        .with_radius(GROUND_SPHERE_RADIUS)
        .with_material(Arc::new(Lambertian::with_texture(checker)))
        .build()
    ));

    let step = 4.0/(GROUND_SPHERE_RADIUS + SPHERE_RADIUS);

    Itertools::cartesian_product(
        StepRange(-step, step, step),
        StepRange(-step, step, step).into_iter().map(|a| a + PI/2.0)
    ).for_each(|(sigma, theta)| {
        let r = (GROUND_SPHERE_RADIUS + SPHERE_RADIUS)*f64::cos(sigma);
        let z = f64::sin(sigma);
        let m: Arc<dyn Material + Send + Sync> = match dist.sample(&mut rng) {
            0 => generate_dielectric(),
            1 => generate_lambertian(&mut rng),
            _ => generate_metal(&mut rng),
        };

        let c = DVec3::new(
            f64::cos(theta),
            f64::sin(theta),
            z
        ).mul_add(r*DVec3::ONE, (-r + 1.0)*DVec3::Y);

        let sphere = Arc::new(SphereBuilder::default()
            .with_center(c)
            .with_radius(SPHERE_RADIUS)
            .with_material(m)
            .build()
        );

        objects.push(sphere);
    });

    BVH::from(objects.as_mut_slice())
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(DVec3::new(0.7, 0.8, 1.0));
    camera_builder.with_look_from(DVec3::new(8.0, 4.0, 10.0));
    camera_builder.with_look_at(DVec3::ZERO);
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(400, 16./9.));
    camera_builder.with_samples_per_pixel(10);
    camera_builder.with_ray_max_bounces(10);
    camera_builder.with_field_of_view((20.*PI)/180.);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects(),
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
