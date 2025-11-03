use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use chrono::Utc;

use clap::Args;

use image::{
    DynamicImage,
    ImageFormat,
    Rgb32FImage,
};

use indicatif::{
    ProgressStyle,
};

use nr_ray_tracer_lib::prelude::*;

use crate::cli::*;
use crate::constants::*;
use crate::scene_config::*;

#[derive(Args)]
pub struct Render {
    pub scene: PathBuf,

    #[command(flatten)]
    image: ImageConfig,

    #[command(flatten)]
    camera: CameraConfig,

    /// Show progress.
    #[arg(short, long)]
    verbose: bool
}

impl Verbosity for Render {
    fn is_verbose(&self) -> bool {
        self.verbose
    }
}

fn render_scene(
    cli: &Render,
    scene: &Scene,
) -> Rgb32FImage {
    let bar = get_progress(cli, "Rendering").map(|bar| {
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
    cli: &Render,
    file: &mut fs::File,
    mut image: Rgb32FImage,
    image_format: ImageFormat,
) -> Result<()> {
    let start = Utc::now();
    let progress = get_spinner(cli, "Exporting");

    gamma_correction(&mut image, cli.image.gamma_value);

    DynamicImage::ImageRgb32F(image)
        .to_rgb8()
        .write_to(file, image_format)?;

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

    Ok(())
}

pub fn run(args: &Render) -> Result<()> {
    let (mut file, format) = args.image.get_file()?;

    let mut scene_config = SceneConfig::try_load_scene(args.scene.as_path())?;

    scene_config.camera.merge_with(&args.camera);

    let scene = scene_config.try_build()?;
    let image = render_scene(args, &scene);

    dump_image(args, &mut file, image, format)
}
