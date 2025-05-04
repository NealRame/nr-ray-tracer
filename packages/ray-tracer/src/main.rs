mod cli;
mod constants;

use std::fs::File;

use clap::{
    CommandFactory,
    Parser,
};
use clap::error::ErrorKind;

use glam::DVec3;

use nr_ray_tracer_lib::prelude::*;

use crate::cli::*;

fn dump_image(cli: &Cli, image: &Image) {
    let overwrite = cli.force_overwrite;
    let filepath = cli.output.clone().unwrap_or("out.ppm".try_into().unwrap());

    let mut file = File::options()
        .create_new(!overwrite)
        .create(true)
        .truncate(true)
        .write(true)
        .open(filepath.as_path())
        .unwrap_or_else(|err| {
            Cli::command().error(ErrorKind::Io, format!(
                "Fail to open '{}' for writing. {}.",
                filepath.to_string_lossy(),
                err.to_string(),
            )).exit();
        });

    write_ppm(image, &mut file)
        .unwrap_or_else(|err| {
            Cli::command().error(ErrorKind::Io, format!(
                "Fail to write image. {}.",
                err.to_string(),
            )).exit();
        });
}

fn ray_color(ray: &Ray, hitable: &impl Hitable) -> DVec3 {
    match hitable.hit(ray, Interval::POSITIVE) {
        Some(hit_record) => {
            (hit_record.normal + DVec3::ONE)/2.0
        },
        _ => {
            let d = ray.get_direction().normalize();
            let a = (d.y + 1.)/2.;

            (1. - a)*DVec3::ONE + a*DVec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let cli = Cli::parse();

    // Image
    let image = cli.image_size.validate();

    // Camera
    let mut camera = CameraBuilder::new()
        .with_eye_at(DVec3::ZERO)
        .with_focal_length(cli.focal_length)
        .with_sample_per_pixels(cli.anti_aliasing)
        .build(image);

    // World
    let world = HitableList::from(vec![
        Box::new(Sphere::new(DVec3::new(0.0,    0.0, -1.0),   0.5)),
        Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Render
    camera.render(&world, ray_color);

    // Dump image
    dump_image(&cli, &camera.take_image());
}
