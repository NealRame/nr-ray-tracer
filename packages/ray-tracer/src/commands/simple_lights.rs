use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use super::render;

fn generate_objects() -> Result<BVH> {
    // const SEED: u64 = 1;
    const GROUND_SPHERE_RADIUS: f64 = 1_000_000.0;
    const SMALL_SPHERE_RADIUS: f64 = 2.0;

    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(GROUND_SPHERE_RADIUS*DVec3::Y)
        .with_radius(GROUND_SPHERE_RADIUS)
        .with_material(Arc::new(Lambertian::with_color(0.4*DVec3::ONE)))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(SMALL_SPHERE_RADIUS*DVec3::Y)
        .with_radius(SMALL_SPHERE_RADIUS)
        .with_material(Arc::new(Lambertian::with_texture(
            Arc::new(MarbleBuilder::default()
                .with_frequency(0.2)
                .build())
        )))
        .build()
    ));

    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(DVec3::new(3.0, 1.0, -2.0))
        .with_u(2.0*DVec3::X)
        .with_v(2.0*DVec3::Y)
        .with_material(Arc::new(DiffuseLight::with_color(DVec3::new(4.0, 2.0, 1.0))))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(7.0*DVec3::Y)
        .with_radius(1.0)
        .with_material(Arc::new(DiffuseLight::with_color(DVec3::new(1.0, 2.0, 4.0))))
        .build()
    ));

    Ok(BVH::from(objects.as_mut_slice()))
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(0.001*DVec3::ONE);
    camera_builder.with_look_from(DVec3::new(26.0, 3.0, 6.0));
    camera_builder.with_look_at(2.0*DVec3::Y);
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(400, 16./9.0));
    camera_builder.with_samples_per_pixel(10);
    camera_builder.with_ray_max_bounces(10);
    camera_builder.with_field_of_view((20.*PI)/180.);
    camera_builder.with_defocus_angle(0.);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects()?,
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
