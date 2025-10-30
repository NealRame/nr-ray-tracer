use std::borrow::Cow;
use std::f64::consts::PI;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

use color_print::{
    cformat,
    cstr,
};

use glam::DVec3;

use image::ImageFormat;

use indicatif::{
    ProgressBar,
    ProgressStyle,
};

use nr_ray_tracer_lib::prelude::*;

use once_cell::sync::Lazy;

use regex::Regex;

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use thiserror::Error;

use crate::constants::*;


#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid vector: '{0}'")]
    InvalidVectorArgument(String),

    #[error("Invalid image ratio: '{0}'")]
    InvalidRatioArgument(String),

    #[error(
        "When '{}' or '{}' are specified, one of '{}', '{}', '{}', '{}' must be specified too.",
        cformat!("<yellow>{}</yellow>", .0),
        cformat!("<yellow>{}</yellow>", .1),
        cformat!("<yellow>{}</yellow>", .2),
        cformat!("<yellow>{}</yellow>", .3),
        cformat!("<yellow>{}</yellow>", .4),
        cformat!("<yellow>{}</yellow>", .5),
    )]
   ImageSizeMissingArg(&'static str,&'static str,&'static str,&'static str,&'static str,&'static str,),

    #[error(
        "When '{}' or '{}' are specified, '{}' or '{}' and '{}' or '{}' are mutually exclusive.",
        cstr!("<yellow>-R,</yellow>"),
        cstr!("<yellow>--aspect-ratio</yellow>"),
        cstr!("<yellow>-W</yellow>"),
        cstr!("<yellow>--width</yellow>"),
        cstr!("<yellow>-H</yellow>"),
        cstr!("<yellow>--height</yellow>"),
    )]
    ImageSizeConflictingArgs
}

fn parse_vector(s: &str) -> std::result::Result<DVec3, CliError> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(\S+),(\S+),(\S+)$").unwrap()
    });

    let captures = RE.captures(s.trim()).ok_or_else(|| CliError::InvalidVectorArgument(s.into()))?;

    match (
        captures.get(1).unwrap().as_str().trim().parse::<f64>(),
        captures.get(2).unwrap().as_str().trim().parse::<f64>(),
        captures.get(3).unwrap().as_str().trim().parse::<f64>(),
    ) {
        (Ok(x), Ok(y), Ok(z),) => {
            Ok(DVec3::new(x, y, z))
        },
        _ => {
            Err(CliError::InvalidVectorArgument(s.into()))
        }
    }
}

fn parse_aspect_ratio(mut s: &str) -> Result<f64, CliError> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(\d+)\s*/\s*(\d+)$").unwrap()
    });

    s = s.trim();

    if let Ok(ratio) = s.trim().parse::<f64>() {
        Ok(ratio)
    } else {
        let captures = RE.captures(s.trim()).ok_or_else(|| CliError::InvalidRatioArgument(s.into()))?;

        let w = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let h = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();

        Ok(w/h)
    }
}

#[derive(clap::Args, Debug)]
#[group(id = "Image")]
pub struct ImageConfig {
    /// Force output overwrite.
    #[arg(
        long,
        short = 'f',
    )]
    pub force_overwrite: bool,

    /// Output file path.
    #[arg(
        long,
        short = 'o',
        value_name = "FILE",
        default_value = "out.png",
    )]
    pub output: PathBuf,

    /// Specify the gamma value.
    #[arg(
        long,
        value_name = "GAMMA_VALUE",
        default_value_t = DEFAULT_IMAGE_GAMMA_VALUE,
    )]
    pub gamma_value: f32,
}

impl ImageConfig {
    pub fn get_file(&self) -> Result<(File, ImageFormat)> {
        let output = self.output.as_path();

        let format = ImageFormat::from_path(output)?;
        let file =
            File::options()
                .create_new(!self.force_overwrite)
                .create(true)
                .truncate(true)
                .write(true)
                .open(output)?
            ;

        Ok((file, format))
    }
}

#[derive(clap::Args, Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[group(id = "camera")]
#[skip_serializing_none]
pub struct CameraConfig {
    /// The image width.
    #[arg(
        env = "NR_RT_CAMERA_WIDTH",
        short = 'W',
        long,
        value_name = "WIDTH",
    )]
    pub width: Option<usize>,

    /// The image height.
    #[arg(
        env = "NR_RT_CAMERA_HEIGHT",
        short = 'H',
        long,
        value_name = "HEIGHT",
    )]
    pub height: Option<usize>,

    /// The image aspect ratio.
    #[arg(
        env = "NR_RT_CAMERA_ASPECT_RATIO",
        long,
        value_name = "ASPECT_RATIO",
        value_parser = parse_aspect_ratio,
    )]
    pub aspect_ratio: Option<f64>,

    /// Background color
    #[arg(
        env = "NR_RT_CAMERA_BACKGROUND_COLOR",
        long,
        value_name = "COLOR",
        value_parser = parse_vector,
    )]
    pub background_color: Option<DVec3>,

    /// Specify the position where the camera is looking at.
    #[arg(
        env = "NR_RT_CAMERA_LOOK_AT",
        long,
        value_name = "POSITION",
        value_parser = parse_vector,
    )]
    pub look_at: Option<DVec3>,

    /// Specify the position from where the camera is looking.
    #[arg(
        env = "NR_RT_CAMERA_LOOK_FROM",
        long,
        value_name = "POSITION",
        value_parser = parse_vector,
    )]
    pub look_from: Option<DVec3>,

    /// Specify the view up direction of the camera.
    #[arg(
        env = "NR_RT_CAMERA_VIEW_UP",
        long, value_name = "POSITION",
        value_parser = parse_vector,
    )]
    pub view_up: Option<DVec3>,

    /// Specify the camera focal length.
    #[arg(
        env = "NR_RT_CAMERA_FOCAL_LENGTH",
        long,
        value_name = "FOCAL_LENGTH",
    )]
    pub focal_length: Option<f64>,

    /// Specify the camera focal length.
    #[arg(
        env = "NR_RT_CAMERA_FIELD_OF_VIEW",
        long,
        value_name = "FOV",
    )]
    pub field_of_view: Option<f64>,

    /// Specify the defocus angle.
    #[arg(
        env = "NR_RT_CAMERA_DEFOCUS_ANGLE",
        long,
        value_name = "DEFOCUS_ANGLE",
    )]
    pub defocus_angle: Option<f64>,

    /// Specify the focus distance.
    #[arg(
        env = "NR_RT_CAMERA_FOCUS_DISTANCE",
        long,
        value_name = "FOCUS_DISTANCE",
    )]
    pub focus_distance: Option<f64>,

    /// Specify how many samples per pixels anti-aliasing will use.
    #[arg(
        env = "NR_RT_CAMERA_SAMPLES_PER_PIXEL",
        long,
        value_name = "SAMPLES_PER_PIXEL",
    )]
    pub samples_per_pixel: Option<usize>,

    /// Maximum ray bounce count.
    #[arg(
        env = "NR_RT_CAMERA_RAY_MAX_BOUNCES",
        long,
        value_name = "COUNT",
    )]
    pub ray_max_bounces: Option<usize>,
}

impl CameraConfig {
    pub fn get_size(&self) -> Result<Option<ImageSize>, CliError> {
        match (self.width, self.height, self.aspect_ratio) {
            (None, None, None) => {
                Ok(None)
            },
            (Some(w), Some(h), None) => {
                Ok(Some(ImageSize::new(w, h)))
            },
            (Some(w), None, Some(r)) => {
                Ok(Some(ImageSize::from_width_and_aspect_ratio(w, r)))
            },
            (None, Some(h), Some(r)) => {
                Ok(Some(ImageSize::from_height_and_aspect_ratio(h, r)))
            },
            (Some(_), None, None) => {
                Err(CliError::ImageSizeMissingArg(
                    "-W", "--width",
                    "-H", "--height",
                    "-R", "--aspect-ratio",
                ))
            },
            (None, Some(_), None) => {
                Err(CliError::ImageSizeMissingArg(
                    "-H", "--height",
                    "-W", "--width",
                    "-R", "--aspect-ratio",
                ))
            },
            (None, None, Some(_)) => {
                Err(CliError::ImageSizeMissingArg(
                    "-R", "--aspect-ratio",
                    "-W", "--width",
                    "-H", "--height",
                ))
            },
            (Some(_), Some(_), Some(_)) => {
                Err(CliError::ImageSizeConflictingArgs)
            },
        }
    }
}

impl CameraConfig {
    pub fn merge_with(&mut self, other: &Self) {
        if let Some(color) = other.background_color {
            self.background_color.replace(color);
        }
        if let Some(width) = other.width {
            self.width.replace(width);
        }
        if let Some(height) = other.height {
            self.height.replace(height);
        }
        if let Some(field_of_view) = other.field_of_view {
            self.field_of_view.replace(field_of_view);
        }
        if let Some(focus_distance) = other.focus_distance {
            self.focus_distance.replace(focus_distance);
        }
        if let Some(defocus_angle) = other.defocus_angle {
            self.defocus_angle.replace(defocus_angle);
        }
        if let Some(samples_per_pixel) = other.samples_per_pixel {
            self.samples_per_pixel.replace(samples_per_pixel);
        }
        if let Some(ray_max_bounces) = other.ray_max_bounces {
            self.ray_max_bounces.replace(ray_max_bounces);
        }
        if let Some(view_up) = other.view_up {
            self.view_up.replace(view_up);
        }
        if let Some(look_at) = other.look_at {
            self.look_at.replace(look_at);
        }
        if let Some(look_from) = other.look_from {
            self.look_from.replace(look_from);
        }
    }

    pub fn try_update(
        &self,
        config: &mut CameraBuilder,
    ) -> Result<(), CliError> {
        if let Some(image_size) = self.get_size()? {
            config.with_image_size(image_size);
        }

        if let Some(background_color) = self.background_color {
            config.with_background_color(background_color);
        }

        if let Some(field_of_view) = self.field_of_view {
            config.with_field_of_view((field_of_view*PI)/180.0);
        }

        if let Some(focus_distance) = self.focus_distance {
            config.with_focus_dist(focus_distance);
        }

        if let Some(defocus_angle) = self.defocus_angle {
            config.with_defocus_angle((defocus_angle*PI)/180.0);
        }

        if let Some(samples_per_pixel) = self.samples_per_pixel {
            config.with_samples_per_pixel(samples_per_pixel);
        }

        if let Some(ray_max_bounces) = self.ray_max_bounces {
            config.with_ray_max_bounces(ray_max_bounces);
        }

        if let Some(view_up) = self.view_up {
            config.with_view_up(view_up);
        }

        if let Some(look_at) = self.look_at {
            config.with_look_at(look_at);
        }

        if let Some(look_from) = self.look_from {
            config.with_look_from(look_from);
        }

        Ok(())
    }
}

pub trait Verbosity {
    fn is_verbose(&self) -> bool;
}


pub fn get_progress(
    cli: &impl Verbosity,
    prefix: impl Into<Cow<'static, str>>,
) -> Option<ProgressBar> {
    if cli.is_verbose() {
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
    cli: &impl Verbosity,
    prefix: impl Into<Cow<'static, str>>,
) -> Option<ProgressBar> {
    if cli.is_verbose() {
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
