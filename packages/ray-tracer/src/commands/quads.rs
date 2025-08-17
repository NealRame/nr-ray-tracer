use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use super::render;

fn generate_objects() -> Result<BVH> {
    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    let b1 = QuadBuilder::default()
        .with_top_left(DVec3::new(-3.0, -2.0, 5.0))
        .with_u(4.0*DVec3::NEG_Z)
        .with_v(4.0*DVec3::Y)
        .with_material(Arc::new(Lambertian::with_color(DVec3::new(1.0, 0.2, 0.2))))
        .build();
    objects.push(Arc::new(b1));

    let b2 = QuadBuilder::default()
        .with_top_left(DVec3::new(-2.0, -2.0, 0.))
        .with_u(4.0*DVec3::X)
        .with_v(4.0*DVec3::Y)
        .with_material(Arc::new(Lambertian::with_color(DVec3::new(0.2, 1.0, 0.2))))
        .build();
    objects.push(Arc::new(b2));

    let b3 = QuadBuilder::default()
        .with_top_left(DVec3::new(3.0, -2.0, 1.))
        .with_u(4.0*DVec3::Z)
        .with_v(4.0*DVec3::Y)
        .with_material(Arc::new(Lambertian::with_color(DVec3::new(0.2, 0.2, 1.0))))
        .build();
    objects.push(Arc::new(b3));

    let b4 = QuadBuilder::default()
        .with_top_left(DVec3::new(-2.0, 3.0, 1.))
        .with_u(4.0*DVec3::X)
        .with_v(4.0*DVec3::Z)
        .with_material(Arc::new(Lambertian::with_color(DVec3::new(1.0, 0.5, 0.0))))
        .build();
    objects.push(Arc::new(b4));

    let b5 = QuadBuilder::default()
        .with_top_left(DVec3::new(-2.0, -3.0, 5.0))
        .with_u(4.0*DVec3::X)
        .with_v(4.0*DVec3::NEG_Z)
        .with_material(Arc::new(Lambertian::with_color(DVec3::new(0.2, 0.8, 0.8))))
        .build();
    objects.push(Arc::new(b5));

    Ok(BVH::from(objects.as_mut_slice()))
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(DVec3::new(0.7, 0.8, 1.0));
    camera_builder.with_look_from(9.*DVec3::Z);
    camera_builder.with_look_at(DVec3::ZERO);
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(400, 1.));
    camera_builder.with_samples_per_pixel(10);
    camera_builder.with_ray_max_bounces(10);
    camera_builder.with_field_of_view((80.*PI)/180.);
    camera_builder.with_defocus_angle(0.);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects()?,
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
