use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

use chrono::Utc;

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
use crate::constants::*;

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub image: ImageArgs,

    #[command(flatten)]
    pub camera: CameraArgs,

    /// Force output overwrite.
    #[arg(short = 'f', long)]
    force_overwrite: bool,

    /// Output file path.
    #[arg(short = 'o', long, value_name = "FILE", default_value = "out.png")]
    output: PathBuf,

    /// Show progress.
    #[arg(short, long)]
    pub verbose: bool
}

impl Args {
    pub fn get_file(&self) -> Result<(fs::File, ImageFormat)> {
        let format = ImageFormat::from_path(self.output.as_path())?;
        let file =
            fs::File::options()
                .create_new(!self.force_overwrite)
                .create(true)
                .truncate(true)
                .write(true)
                .open(self.output.as_path())?
            ;

        Ok((file, format))
    }

    pub fn get_progress(
        &self,
        prefix: impl Into<Cow<'static, str>>,
    ) -> Option<ProgressBar> {
        if self.verbose {
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

    pub fn get_spinner(
        &self,
        prefix: impl Into<Cow<'static, str>>,
    ) -> Option<ProgressBar> {
        if self.verbose {
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
}

pub fn render_scene(
    cli: &Args,
    scene: &Scene,
) -> Rgb32FImage {
    let bar = cli.get_progress("Rendering").map(|bar| {
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

pub fn dump_image(
    cli: &Args,
    file: &mut fs::File,
    mut image: Rgb32FImage,
    image_format: ImageFormat,
) -> Result<()> {
    let start = Utc::now();
    let progress = cli.get_spinner("Exporting");

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
