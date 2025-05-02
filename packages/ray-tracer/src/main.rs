mod cli;
mod constants;

use std::fs::File;

use clap::{
    CommandFactory,
    Parser,
};
use clap::error::ErrorKind;

use glam::{
    DVec3,
    DVec4,
    U8Vec4,
};
use glam::swizzles::Vec4Swizzles;

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
) -> bool {
    let dir = ray.get_direction();
    let eye = ray.get_origin();
    let ec = center - eye;

    let a = dir.dot(dir);
    let b = -2.0*ec.dot(dir);
    let c = ec.dot(ec) - radius*radius;

    b*b - 4.0*a*c >= 0.0
}

fn ray_color(ray: &Ray) -> U8Vec4 {
    if hit_sphere(ray, &DVec3::new(0., 0., -1.), 0.5) {
        U8Vec4::new(255, 0, 0, 255)
    } else {
        let d = ray.get_direction().normalize();
        let a = (d.y + 1.)/2.;
        let c = DVec4::ONE.xyzw().with_xyz((1. - a)*DVec3::ONE + a*DVec3::new(0.5, 0.7, 1.0));

        (255.*c).as_u8vec4()
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
