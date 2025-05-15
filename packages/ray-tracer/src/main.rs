mod cli;
mod constants;

use std::f64::consts::PI;
use std::fs::File;
use std::borrow::Cow;
use std::time::Duration;

use chrono::Utc;

use clap::{
    CommandFactory,
    Parser,
};
use clap::error::ErrorKind;

use glam::DVec3;

use image::{
    DynamicImage,
    ImageFormat,
    Rgb32FImage,
};

use indicatif::{
    ProgressBar,
    ProgressStyle,
};

use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::{self, Rng};

use nr_ray_tracer_lib::prelude::*;

use crate::cli::*;

const PROGRESS_TEMPLATE: &'static str = "{prefix:>10} - [{bar:40}] {percent:>3}%";
const SPINNER_TEMPLATE: &'static str = "{prefix:>10} - {spinner:40}";
const PROGRESS_TEMPLATE_FINISHED: &'static str = "{prefix:>10} - {msg}";

fn get_progress(
    cli: &Cli,
    prefix: impl Into<Cow<'static, str>>,
) -> Option<ProgressBar> {
    if cli.verbose {
        ProgressStyle::with_template(PROGRESS_TEMPLATE)
            .map(|style| style.progress_chars("#>-"))
            .map(|style| {
                let bar =
                    ProgressBar::no_length()
                        .with_style(style)
                        .with_prefix(prefix);
                bar
            })
            .ok()
    } else {
        None
    }
}

fn get_spinner(
    cli: &Cli,
    prefix: impl Into<Cow<'static, str>>,
) -> Option<ProgressBar> {
    if cli.verbose {
        ProgressStyle::with_template(SPINNER_TEMPLATE)
            .map(|style| {
                let bar =
                    ProgressBar::new_spinner()
                        .with_style(style)
                        .with_prefix(prefix);

                bar.enable_steady_tick(Duration::from_millis(100));
                bar
            })
            .ok()
    } else {
        None
    }
}

fn render_scene(
    cli: &Cli,
    scene: &Scene,
) -> Rgb32FImage {
    let bar = get_progress(&cli, "Rendering").map(|bar| {
        bar.set_position(0);
        bar.set_length(scene.camera.get_image_size().get_pixel_count() as u64);
        bar
    });

    let start = Utc::now();

    let image = scene.render(bar.as_ref().map(|bar| || bar.inc(1)));

    let stop = Utc::now();
    let duration = stop - start;

    if let Some(bar) = bar.as_ref() {
        bar.set_style(ProgressStyle::with_template(PROGRESS_TEMPLATE_FINISHED).unwrap());
        bar.finish_with_message(format!("Done in {}.{:0<3} secs",
            duration.num_seconds(),
            duration.num_milliseconds()%1000,
        ));
    }
    image
}

fn dump_image(
    cli: &Cli,
    file: &mut File,
    mut image: Rgb32FImage,
    image_format: ImageFormat,
) {
    let start = Utc::now();
    let progress = get_spinner(&cli, "Exporting");

    gamma_correction(&mut image, cli.gamma_value);

    DynamicImage::ImageRgb32F(image)
        .to_rgb8()
        .write_to(file, image_format)
        .unwrap_or_else(|err| {
            Cli::command().error(ErrorKind::Io, format!(
                "Fail to write image. {}.",
                err.to_string(),
            )).exit();
        });

    let stop = Utc::now();
    let duration = stop - start;

    // rgb_image
    if let Some(bar) = progress.as_ref() {
        bar.set_style(ProgressStyle::with_template(PROGRESS_TEMPLATE_FINISHED).unwrap());
        bar.finish_with_message(format!("Done in {}.{:0<3} secs",
            duration.num_seconds(),
            duration.num_milliseconds()%1000,
        ));
    }
}

fn main() {
    let cli = Cli::parse();

    // Check image size config
    let image_size = cli.image_size.check();

    // Get output file descriptor
    let (mut file, format) = cli.output.check();

    // Initialize world
    let mut objects = vec![
        Object::Sphere(Sphere::new(
            1000.0*DVec3::NEG_Y,
            1000.0,
            Material::lambertian_default(),
        )),
    ];

    let mut rng = rand::rng();
    let materials = ["lambertian", "metal", "glass"];

    let material_weights = [80, 15, 5];
    let dist = WeightedIndex::new(&material_weights).unwrap();

    for a in -11..=11 {
        for b in -11..=11 {
            let center = DVec3::new(
                a as f64 + 0.9*rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9*rng.random_range(0.0..1.0),
            );

            match materials[dist.sample(&mut rng)] {
                "lambertian" => {
                    objects.push(Object::Sphere(Sphere::new(
                        center, 0.2,
                        Material::lambertian_from_rng(&mut rng)
                    )));
                },
                "metal" => {
                    objects.push(Object::Sphere(Sphere::new(
                        center, 0.2,
                        Material::metal_from_rng(&mut rng),
                    )));
                },
                "glass" => {
                    objects.push(Object::Sphere(Sphere::new(
                        center, 0.2,
                        Material::dielectric_default(),
                    )));
                },
                _ => unreachable!()
            }
        }
    }

    objects.push(Object::Sphere(Sphere::new(
        DVec3::new( 0.0, 1.0, 0.0), 1.0,
        Material::dielectric_default(),
    )));
    objects.push(Object::Sphere(Sphere::new(
        DVec3::new(-4.0, 1.0, 0.0), 1.0,
        Material::Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1),
        },
    )));
    objects.push(Object::Sphere(Sphere::new(
        DVec3::new( 4.0, 1.0, 0.0), 1.0,
        Material::Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    // Initialize camera
    let camera_config =
        CameraConfig::default()
            .with_image_size(image_size)
            .with_field_of_view(cli.vfov*(PI/180.))
            .with_focus_dist(cli.focus_distance)
            .with_defocus_angle(cli.defocus_angle*(PI/180.))
            .with_look_from(DVec3::new(13.0,  2.0,  3.0))
            .with_sample_per_pixels(cli.anti_aliasing)
            .clone()
        ;

    // Create scene
    let scene = Scene {
        camera: camera_config.build(),
        objects,
    };

    // Render image
    let image = render_scene(&cli, &scene);

    dump_image(&cli, &mut file, image, format);
}
