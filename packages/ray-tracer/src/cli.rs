use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

use clap::{
    Args,
    CommandFactory,
    Parser,
};
use clap::error::ErrorKind;

use color_print::{
    cformat,
    cstr,
};

use image::ImageFormat;

use indicatif::{
    ProgressBar,
    ProgressStyle,
};

use once_cell::sync::Lazy;

use regex::Regex;

use nr_ray_tracer_lib::prelude::*;

use crate::constants::*;

fn aspect_ratio(s: &str) -> Result<f64, String> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(\d+)\s*/\s*(\d+)$").unwrap()
    });

    if let Ok(ratio) = s.trim().parse::<f64>() {
        Ok(ratio)
    } else {
        RE.captures(s.trim())
            .ok_or(format!("Invalid ratio: '{s}'"))
            .and_then(|caps| {
                let w = caps.get(1).unwrap().as_str().parse::<f64>().unwrap();
                let h = caps.get(2).unwrap().as_str().parse::<f64>().unwrap();

                Ok(w/h)
            })
    }
}

fn report_image_size_missing_arg_error(
    _1: &str, _2: &str, _3: &str, _4: &str, _5: &str, _6: &str,
) -> ! {
    Cli::command().error(ErrorKind::MissingRequiredArgument, format!(
        "When '{}' or '{}' are specified, one of '{}', '{}', '{}', '{}' must be specified too.",
        cformat!("<yellow>{_1}</yellow>"),
        cformat!("<yellow>{_2}</yellow>"),
        cformat!("<yellow>{_3}</yellow>"),
        cformat!("<yellow>{_4}</yellow>"),
        cformat!("<yellow>{_5}</yellow>"),
        cformat!("<yellow>{_6}</yellow>"),
    )).exit();
}

fn report_image_size_conflicting_args_error() -> ! {
    Cli::command().error(ErrorKind::ArgumentConflict, format!(
        "When '{}' or '{}' are specified, '{}' or '{}' and '{}' or '{}' are mutually exclusive.",
        cstr!("<yellow>-R,</yellow>"),
        cstr!("<yellow>--aspect-ratio</yellow>"),
        cstr!("<yellow>-W</yellow>"),
        cstr!("<yellow>--width</yellow>"),
        cstr!("<yellow>-H</yellow>"),
        cstr!("<yellow>--height</yellow>"),
    )).exit();
}

#[derive(Args)]
#[group()]
pub struct CliImage {
    /// The image width.
    #[arg(long, value_name = "WIDTH")]
    width: Option<usize>,

    /// The image height.
    #[arg(long, value_name = "HEIGHT")]
    height: Option<usize>,

    /// The image aspect ratio.
    #[arg(
        long,
        value_name = "ASPECT_RATIO",
        value_parser = aspect_ratio,
    )]
    aspect_ratio: Option<f64>,

    /// Specify the gamma value.
    #[arg(
        long,
        value_name = "GAMMA_VALUE",
        default_value_t = DEFAULT_IMAGE_GAMMA_VALUE,
    )]
    pub gamma_value: f32,

    /// Specify how many samples per pixels anti-aliasing will use.
    #[arg(
        long,
        value_name = "SAMPLES_PER_PIXELS",
        default_value_t = DEFAULT_SAMPLES_PER_PIXELS
    )]
    pub anti_aliasing: usize,

    /// Maximum ray bounce count.
    #[arg(
        long,
        value_name = "MAX_DEPTH",
        default_value_t = DEFAULT_RAY_MAX_DEPTH
    )]
    pub max_depth: usize,

    /// Force output overwrite.
    #[arg(long)]
    force_overwrite: bool,

    /// Output file path.
    #[arg(long, value_name = "FILE")]
    output: Option<PathBuf>,
}

impl CliImage {
    pub fn get_size(&self) -> ImageSize {
        match (self.width, self.height, self.aspect_ratio) {
            (None, None, None) => {
                ImageSize::new(DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT)
            },
            (Some(w), Some(h), None) => {
                ImageSize::new(w, h)
            },
            (Some(w), None, Some(r)) => {
                ImageSize::from_width_and_aspect_ratio(w, r)
            },
            (None, Some(h), Some(r)) => {
                ImageSize::from_height_and_aspect_ratio(h, r)
            },
            (Some(_), None, None) => {
                report_image_size_missing_arg_error(
                    "-W", "--width",
                    "-H", "--height",
                    "-R", "--aspect-ratio",
                );
            },
            (None, Some(_), None) => {
                report_image_size_missing_arg_error(
                    "-H", "--height",
                    "-W", "--width",
                    "-R", "--aspect-ratio",
                );
            },
            (None, None, Some(_)) => {
                report_image_size_missing_arg_error(
                    "-R", "--aspect-ratio",
                    "-W", "--width",
                    "-H", "--height",
                );
            },
            (Some(_), Some(_), Some(_)) => {
                report_image_size_conflicting_args_error();
            },
        }
    }
}

impl CliImage {
    pub fn get_file(&self) -> (File, ImageFormat) {
        let overwrite = self.force_overwrite;
        let filepath = self.output.clone().unwrap_or("out.bmp".try_into().unwrap());

        let format =
            ImageFormat::from_path(filepath.as_path())
                .unwrap_or_else(|err| {
                    Cli::command().error(
                        ErrorKind::InvalidValue,
                        err.to_string(),
                    ).exit();
                });

        let file =
            File::options()
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

        (file, format)
    }
}

#[derive(Args)]
#[group()]
pub struct CliCamera{
    /// Specify the camera focal length.
    #[arg(
        long,
        value_name = "FOCAL_LENGTH",
        default_value_t = DEFAULT_CAMERA_FOCAL_LENGTH
    )]
    pub focal_length: f64,

    /// Specify the camera focal length.
    #[arg(
        long,
        value_name = "FOV",
        default_value_t = DEFAULT_CAMERA_FIELD_OF_VIEW
    )]
    pub field_of_view: f64,

    /// Specify the defocus angle.
    #[arg(
        long,
        value_name = "DEFOCUS_ANGLE",
        default_value_t = DEFAULT_CAMERA_DEFOCUS_ANGLE
    )]
    pub defocus_angle: f64,

    /// Specify the focus distance.
    #[arg(
        long,
        value_name = "FOCUS_DISTANCE",
        default_value_t = DEFAULT_CAMERA_FOCUS_DISTANCE
    )]
    pub focus_distance: f64,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub scene: PathBuf,

    #[command(flatten)]
    pub image: CliImage,

    #[command(flatten)]
    pub camera: CliCamera,

    /// Show progress.
    #[arg(short, long)]
    pub verbose: bool
}

impl Cli {
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
