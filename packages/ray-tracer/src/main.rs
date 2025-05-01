use std::fs::File;
use std::path::PathBuf;

use itertools::Itertools;

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

use nr_ray_tracer_lib::ppm::write_ppm;

const DEFAULT_IMAGE_WIDTH: usize = 300;
const DEFAULT_IMAGE_HEIGHT: usize = 200;

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
    Cli::command().error(ErrorKind::ArgumentConflict, format!(
        "When '{}' or '{}' are specified, one of '{}', '{}', '{}', '{}' must be specified too.",
        cformat!(r#"<yellow>{_1}</yellow>"#),
        cformat!(r#"<yellow>{_2}</yellow>"#),
        cformat!(r#"<yellow>{_3}</yellow>"#),
        cformat!(r#"<yellow>{_4}</yellow>"#),
        cformat!(r#"<yellow>{_5}</yellow>"#),
        cformat!(r#"<yellow>{_6}</yellow>"#),
    )).exit();
}

fn report_image_size_conflicting_args_error() -> ! {
    Cli::command().error(ErrorKind::ArgumentConflict, format!(
        "When '{}' or '{}' are specified, '{}' or '{}' and '{}' or '{}' are mutually exclusive.",
        cstr!(r#"<yellow>-R,</yellow>"#),
        cstr!(r#"<yellow>--aspect-ratio</yellow>"#),
        cstr!(r#"<yellow>-W</yellow>"#),
        cstr!(r#"<yellow>--width</yellow>"#),
        cstr!(r#"<yellow>-H</yellow>"#),
        cstr!(r#"<yellow>--height</yellow>"#),
    )).exit();
}


#[derive(Args)]
#[group()]
struct ImageSize {
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

impl ImageSize {
    fn validate(&self) -> (usize, usize, f64) {
        let (w, h) = match (self.width, self.height, self.aspect_ratio) {
            (None, None, None) => (DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT),
            (Some(w), Some(h), None) => (w, h),
            (Some(w), None, Some(r)) => (w, (((w as f64)/r) as usize).max(1)),
            (None, Some(h), Some(r)) => ((((h as f64)*r) as usize).max(1), h),
            (Some(_), None, None) => report_image_size_missing_arg_error(
                "-W", "--width",
                "-H", "--height",
                "-R", "--aspect-ratio",
            ),
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
        };

        (w, h, (w as f64)/(h as f64))
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    image_size: ImageSize,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Force output overwrite
    #[arg(short, long)]
    force_overwrite: bool,
}

fn main() {
    let cli = Cli::parse();

    let (width, height, _) = cli.image_size.validate();

    let pixels: Vec<u8> =
        Itertools::cartesian_product(0..height, 0..width)
            .map(|(y, x)| {
                let r = 255.*(x as f64)/((width - 1).max(1) as f64);
                let g = 255.*(y as f64)/((height - 1).max(1) as f64);
                let b = 255. - r;

                vec![r as u8, g as u8, b as u8, 255u8]
            })
            .flatten()
            .collect();

    let mut ppm_out = File::options()
        .create(true)
        .create_new(if cli.force_overwrite { false } else { true })
        .truncate(if cli.force_overwrite { true } else { false })
        .write(true)
        .open(cli.output.unwrap_or("out.ppm".try_into().unwrap()))
        .expect("Fail to open {} for writing");

    write_ppm(&mut ppm_out, width, height, pixels.as_slice())
        .expect("Fail to write ppm");
}
