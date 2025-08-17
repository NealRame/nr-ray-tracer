use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use super::render;

fn generate_objects() -> Result<BVH> {
    const GROUND_SPHERE_RADIUS: f64 = 1000.0;

    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    let earth_texture = Arc::new(Image::try_from_path("scenes/textures/earth.jpg")?);
    let moon_texture = Arc::new(Image::try_from_path("scenes/textures/moon.jpg")?);

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(GROUND_SPHERE_RADIUS*DVec3::NEG_Y)
        .with_radius(GROUND_SPHERE_RADIUS)
        .with_material(Arc::new(Lambertian::with_color(0.5*DVec3::ONE)))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(0., 10., 0.))
        .with_radius(10.)
        .with_material(Arc::new(Lambertian::with_texture(earth_texture)))
        .build()
    ));

    objects.push(Arc::new(SphereBuilder::default()
        .with_center(DVec3::new(-12., 12., -20.))
        .with_radius(3.)
        .with_material(Arc::new(Lambertian::with_texture(moon_texture)))
        .build()
    ));

    Ok(BVH::from(objects.as_mut_slice()))
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(DVec3::new(0.7, 0.8, 1.0));
    camera_builder.with_look_from(DVec3::new(60.0, 20.0, 3.0));
    camera_builder.with_look_at(10.*DVec3::Y);
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(400, 16./9.));
    camera_builder.with_samples_per_pixel(10);
    camera_builder.with_ray_max_bounces(10);
    camera_builder.with_field_of_view((20.*PI)/180.);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects()?,
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
