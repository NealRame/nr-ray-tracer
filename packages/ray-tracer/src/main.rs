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

fn hit_sphere(
    ray: &Ray,
    center: &DVec3,
    radius: f64,
) -> f64 {
    let dir = ray.get_direction();
    let eye = ray.get_origin();
    let ec = center - eye;

    let a = dir.dot(dir);
    let b = -2.0*ec.dot(dir);
    let c = ec.dot(ec) - radius*radius;

    match b*b - 4.0*a*c {
        discriminant if discriminant >= 0.0 => {
            // return the smallest t i.d. the closest point
            (-b - discriminant.sqrt())/(2.0*a)
        },
        _ => -1.0
    }
}

fn ray_color(ray: &Ray) -> DVec3 {
    let sphere_center = DVec3::new(0., 0., -1.);

    match hit_sphere(ray, &sphere_center, 0.5) {
        t if t >= 0.0 => {
            let p = ray.at(t);
            let n = (p - sphere_center).normalize();

            (n + DVec3::ONE)/2.0
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
    let mut camera = Camera::new_with_image(image, DVec3::ZERO, cli.focal_length);

    // Render
    camera.map(|ray, _| {
        ray_color(&ray)
    });

    // Dump image
    dump_image(&cli, &camera.take_image());
}
