use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use super::render;

fn generate_objects() -> Result<BVH> {
    const GROUND_SPHERE_RADIUS: f64 = 1_000_000.0;

    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(GROUND_SPHERE_RADIUS*DVec3::NEG_Y)
        .with_radius(GROUND_SPHERE_RADIUS)
        .with_material(Arc::new(Lambertian::with_color(0.4*DVec3::ONE)))
        .build()
    ));

    let perlin = Arc::new(PerlinRidgedNoiseBuilder::default()
        .with_octaves(8)
        .with_frequency(0.2)
        .build()
    );

    let marble = Arc::new(MarbleBuilder::default()
        .with_frequency(0.2)
        .build()
    );

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(0., 10., 0.))
        .with_radius(10.)
        .with_material(Arc::new(MetalBuilder::default()
            .with_texture(perlin)
            .with_fuzz(0.05)
            .build()
        ))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(-40., 10., 20.))
        .with_radius(10.)
        .with_material(Arc::new(Lambertian::with_texture(marble)))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(30., 10., -20.))
        .with_radius(10.)
        .with_material(Arc::new(MetalBuilder::default()
            .with_color(DVec3::new(1.0, 0.5, 0.65))
            .with_fuzz(0.9)
            .build()
        ))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(10., 10., 25.))
        .with_radius(10.)
        .with_material(Arc::new(MetalBuilder::default()
            .with_color(DVec3::new(0.23, 0.51, 0.88))
            .with_fuzz(0.8)
            .build()
        ))
        .build()
    ));

    Ok(BVH::from(objects.as_mut_slice()))
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(DVec3::new(0.7, 0.8, 1.0));
    camera_builder.with_look_from(70.*DVec3::X + 30.*DVec3::Y);
    camera_builder.with_look_at(10.*DVec3::Y + 2.*DVec3::NEG_Z);
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(400, 16./9.));
    camera_builder.with_samples_per_pixel(10);
    camera_builder.with_ray_max_bounces(10);
    camera_builder.with_field_of_view((30.*PI)/180.);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects()?,
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
