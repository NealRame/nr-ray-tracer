mod cli;
mod constants;

use clap::{
    CommandFactory,
    Parser,
};
use clap::error::ErrorKind;

use glam::DVec3;

use indicatif::{ProgressBar, ProgressStyle};

use nr_ray_tracer_lib::prelude::*;

use crate::cli::*;

fn main() {
    let cli = Cli::parse();

    let progress = if cli.verbose {
        ProgressStyle::with_template("{prefix} [{bar:40}] {percent:>3}%")
            .map(|style| style.progress_chars("#>-"))
            .map(|style| ProgressBar::no_length().with_style(style))
            .ok()
    } else {
        None
    };

    // Image size
    let image_size = cli.image_size.check();

    // Image Output
    let mut file = cli.output.check();

    // Camera
    let mut camera = CameraBuilder::new(image_size)
        .with_eye_at(DVec3::ZERO)
        .with_focal_length(cli.focal_length)
        .with_sample_per_pixels(cli.anti_aliasing)
        .build();

    // World
    let world = HitableList::from(vec![
        Box::new(Sphere::new(DVec3::new(0.0,    0.0, -1.0),   0.5)),
        Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Render
    camera.render(&world, progress.clone().map(|bar| bar.with_prefix("Rendering")));

    // Dump image
    camera.dump(&mut file, progress.clone().map(|bar| bar.with_prefix("Exporting")))
        .unwrap_or_else(|err| {
            Cli::command().error(ErrorKind::Io, format!(
                "Fail to write image. {}.",
                err.to_string(),
            )).exit();
        });
}
