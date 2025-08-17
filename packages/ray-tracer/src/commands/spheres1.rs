use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use itertools::Itertools;

use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;

use nr_ray_tracer_lib::prelude::*;

use super::render;

fn generate_objects() -> BVH {
    const SEED: u64 = 1;
    const GROUND_SPHERE_RADIUS: f64 = 100000.0;
    const SMALL_SPHERE_RADIUS: f64 = 0.2;
    const LARGE_SPHERE_RADIUS: f64 = 1.0;
    const MATERIAL_DISTRIBUTION: &[usize; 3] = &[5, 80, 15];

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

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(GROUND_SPHERE_RADIUS*DVec3::NEG_Y)
        .with_radius(GROUND_SPHERE_RADIUS)
        .with_material(Arc::new(Lambertian::with_color(0.5*DVec3::ONE)))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new( 0.0, LARGE_SPHERE_RADIUS, 0.0))
        .with_radius(LARGE_SPHERE_RADIUS)
        .with_material(generate_dielectric())
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(-4.0, LARGE_SPHERE_RADIUS, 0.0))
        .with_radius(LARGE_SPHERE_RADIUS)
        .with_material(generate_lambertian(&mut rng))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new( 4.0, LARGE_SPHERE_RADIUS, 0.0))
        .with_radius(LARGE_SPHERE_RADIUS)
        .with_material(generate_metal(&mut rng))
        .build()
    ));

    for (a, b) in Itertools::cartesian_product(-11..11, -11..11) {
        let material: Arc<dyn Material + Send + Sync> = match dist.sample(&mut rng) {
            0 => generate_dielectric(),
            1 => generate_lambertian(&mut rng),
            _ => generate_metal(&mut rng),
        };

        let center = DVec3::new(
            a as f64 + 0.9*rng.random::<f64>(),
            SMALL_SPHERE_RADIUS,
            b as f64 + 0.9*rng.random::<f64>(),
        );

        let sphere = Arc::new(SphereBuilder::default()
            .with_center(center)
            .with_radius(SMALL_SPHERE_RADIUS)
            .with_material(material)
            .build()
        );

        objects.push(sphere);
    }

    BVH::from(objects.as_mut_slice())
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(DVec3::new(0.7, 0.8, 1.0));
    camera_builder.with_look_from(DVec3::new(13.0, 2.0, 3.0));
    camera_builder.with_look_at(DVec3::ZERO);
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(400, 16./9.));
    camera_builder.with_samples_per_pixel(100);
    camera_builder.with_ray_max_bounces(50);
    camera_builder.with_field_of_view((20.*PI)/180.);
    camera_builder.with_focus_dist(10.0);
    camera_builder.with_defocus_angle(0.01);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects(),
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
