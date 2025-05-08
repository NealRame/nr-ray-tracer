mod cli;
mod constants;

use std::fs::File;
use std::borrow::Cow;
use std::sync::Arc;
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

fn render_image(
    cli: &Cli,
    camera: &Camera,
    world: &HitableList,
) -> Rgb32FImage {
    let start = Utc::now();

    let progress = get_progress(&cli, "Rendering");
    let image = camera.render(world, progress.as_ref());

    let stop = Utc::now();
    let duration = stop - start;

    if let Some(bar) = progress.as_ref() {
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
    let lambertian_1 = Arc::new(Lambertian::new(DVec3::new(0.259, 0.259, 0.259)));
    let lambertian_2 = Arc::new(Lambertian::new(DVec3::new(0.878, 0.878, 0.878)));

    let metal_1 = Arc::new(Metal::new(DVec3::new(0.901, 0.231, 0.184)));
    // let metal_2 = Arc::new(Metal::new(DVec3::new(0.204, 0.514, 0.851)));

    let metal_2 = Arc::new(Metal::new(DVec3::new(0.220, 0.412, 0.620)));

    let world = HitableList::from(vec![
        Box::new(Sphere::new(DVec3::new( 0.0, -100.5, -1.0), 100.0, lambertian_1.clone())),
        Box::new(Sphere::new(DVec3::new( 0.0,    0.0, -1.2),   0.5, lambertian_2.clone())),
        Box::new(Sphere::new(DVec3::new(-1.0,    0.0, -1.0),   0.5, metal_1.clone())),
        Box::new(Sphere::new(DVec3::new( 1.0,    0.0, -1.0),   0.5, metal_2.clone())),
    ]);

    // Initialize camera
    let camera = CameraBuilder::new(image_size)
        .with_eye_at(DVec3::ZERO)
        .with_focal_length(cli.focal_length)
        .with_sample_per_pixels(cli.anti_aliasing)
        .build();

    // Render image
    let image = render_image(&cli, &camera, &world);

    dump_image(&cli, &mut file, image, format);
}
