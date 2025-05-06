use std::fs::File;
use std::path::PathBuf;

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
pub struct CliImageSize {
    /// The image width
    #[arg(short = 'W', long, value_name = "WIDTH")]
    width: Option<usize>,

    /// The image height
    #[arg(short = 'H', long, value_name = "HEIGHT")]
    height: Option<usize>,

    /// The image aspect ratio
    #[arg(short = 'R', long, value_name = "ASPECT_RATIO", value_parser = aspect_ratio)]
    aspect_ratio: Option<f64>,
}

impl CliImageSize {
    pub fn check(&self) -> ImageSize {
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


#[derive(Args)]
#[group()]
pub struct CliOutput {
    /// Force output overwrite
    #[arg(short, long)]
    pub force_overwrite: bool,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

impl CliOutput {
    pub fn check(&self) -> File {
        let overwrite = self.force_overwrite;
        let filepath = self.output.clone().unwrap_or("out.ppm".try_into().unwrap());

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
            })
    }
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub image_size: CliImageSize,

    #[command(flatten)]
    pub output: CliOutput,

    /// How many samples per pixels anti-aliasing will use
    #[arg(
        short = 'A', long,
        value_name = "SAMPLES_PER_PIXELS",
        default_value_t = DEFAULT_SAMPLES_PER_PIXELS
    )]
    pub anti_aliasing: usize,

    /// Focal length
    #[arg(
        short = 'F', long,
        value_name = "FOCAL_LENGTH",
        default_value_t = DEFAULT_CAMERA_FOCAL_LENGTH
    )]
    pub focal_length: f64,

    /// Maximum ray bounce count
    #[arg(
        short = 'D', long,
        value_name = "MAX_DEPTH",
        default_value_t = DEFAULT_RAY_MAX_DEPTH
    )]
    pub max_depth: usize,

    /// Show progress
    #[arg(short, long)]
    pub verbose: bool
}
