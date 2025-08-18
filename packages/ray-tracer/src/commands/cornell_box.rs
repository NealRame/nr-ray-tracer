use std::f64::consts::PI;
use std::sync::Arc;

use anyhow::Result;

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use super::render;

fn generate_box(
    a: DVec3, b: DVec3,
    material: Arc<dyn Material + Send + Sync>,
) -> BVH {
    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    let [min_x, min_y, min_z] = a.min(b).to_array();
    let [max_x, max_y, max_z] = a.max(b).to_array();

    let dx = (max_x - min_x)*DVec3::X;
    let dy = (max_y - min_y)*DVec3::Y;
    let dz = (max_z - min_z)*DVec3::Z;

    objects.push(Arc::new(QuadBuilder::default()
        .with_material(material.clone())
        .with_top_left(DVec3::new(min_x, min_y, max_z))
        .with_u(dx)
        .with_v(dy)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_material(material.clone())
        .with_top_left(DVec3::new(max_x, min_y, max_z))
        .with_u(-dz)
        .with_v(dy)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_material(material.clone())
        .with_top_left(DVec3::new(max_x, min_y, min_z))
        .with_u(-dx)
        .with_v(dy)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_material(material.clone())
        .with_top_left(DVec3::new(min_x, min_y, min_z))
        .with_u(dz)
        .with_v(dy)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_material(material.clone())
        .with_top_left(DVec3::new(min_x, max_y, max_z))
        .with_u(dx)
        .with_v(-dz)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_material(material.clone())
        .with_top_left(DVec3::new(min_x, min_y, min_z))
        .with_u(dx)
        .with_v(dz)
        .build()
    ));

    BVH::from(objects.as_mut_slice())
}

fn generate_cornel_box(objects: &mut Vec::<Arc<dyn Hitable + Send + Sync>>) {
    let red = Arc::new(Lambertian::with_color(DVec3::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::with_color(DVec3::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::with_color(DVec3::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::with_color(DVec3::new(15., 15., 15.)));

    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(555.0*DVec3::X)
        .with_u(555.0*DVec3::Y)
        .with_v(555.0*DVec3::Z)
        .with_material(green)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(DVec3::ZERO)
        .with_u(555.0*DVec3::Y)
        .with_v(555.0*DVec3::Z)
        .with_material(red)
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(DVec3::ZERO)
        .with_u(555.0*DVec3::X)
        .with_v(555.0*DVec3::Z)
        .with_material(white.clone())
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(555.0*DVec3::ONE)
        .with_u(555.0*DVec3::NEG_X)
        .with_v(555.0*DVec3::NEG_Z)
        .with_material(white.clone())
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(555.0*DVec3::Z)
        .with_u(555.0*DVec3::X)
        .with_v(555.0*DVec3::Y)
        .with_material(white.clone())
        .build()
    ));
    objects.push(Arc::new(QuadBuilder::default()
        .with_top_left(DVec3::new(343.0, 554.0, 332.0))
        .with_u(130.0*DVec3::NEG_X)
        .with_v(105.0*DVec3::NEG_Z)
        .with_material(light)
        .build()
    ));
}

fn generate_objects() -> Result<BVH> {
    let mut objects = Vec::<Arc<dyn Hitable + Send + Sync>>::new();

    generate_cornel_box(&mut objects);

    objects.push(
        Arc::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(generate_box(
                    DVec3::ZERO,
                    DVec3::new(165.0, 165.0, 165.0),
                    Arc::new(Lambertian::with_color(DVec3::new(0.93,1.00,0.60))),
                )),
                -18.0*PI/180.0,
            )),
            DVec3::new(130.0, 0.0, 65.0),
        ))
    );

    objects.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(generate_box(
                DVec3::ZERO,
                DVec3::new(165.0, 330.0, 165.0),
                Arc::new(
                    MetalBuilder::default()
                        .with_color(DVec3::new(0.00, 0.82, 1.00))
                        .with_fuzz(0.025)
                        .build()
                ),
            )),
            15.0*PI/180.0
        )),
        DVec3::new(265.0, 0.0, 295.0),
    )));

    Ok(BVH::from(objects.as_mut_slice()))
}

pub fn run(args: &render::Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let mut camera_builder = CameraBuilder::default();

    camera_builder.with_background_color(0.01*DVec3::ONE);
    camera_builder.with_look_from(DVec3::new(278.0, 278.0, -800.0));
    camera_builder.with_look_at(DVec3::new(278.0, 278.0, 0.0));
    camera_builder.with_image_size(ImageSize::from_width_and_aspect_ratio(600, 1.0));
    camera_builder.with_samples_per_pixel(100);
    camera_builder.with_ray_max_bounces(50);
    camera_builder.with_field_of_view((40.*PI)/180.);
    camera_builder.with_defocus_angle(0.);

    args.camera.try_update(&mut camera_builder)?;

    let scene = Scene {
        camera: camera_builder.build(),
        objects: generate_objects()?,
    };

    let image = render::render_scene(&args, &scene);

    render::dump_image(&args, &mut file, image, format)
}
