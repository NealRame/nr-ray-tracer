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

use glam::{
    DVec3,
    DVec4,
    U8Vec4,
};
use glam::swizzles::Vec4Swizzles;

use once_cell::sync::Lazy;

use regex::Regex;

use nr_ray_tracer_lib::ray::Ray;
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

    /// Focal length
    #[arg(short = 'F', long, value_name = "FOCAL_LENGTH", default_value_t = 1.0)]
    focal_length: f64,

    /// Force output overwrite
    #[arg(short, long)]
    force_overwrite: bool,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn ray_color(ray: &Ray) -> U8Vec4 {
    let d = ray.get_direction().normalize();
    let a = (d.y + 1.)/2.;
    let c = DVec4::ONE.xyzw().with_xyz((1. - a)*DVec3::ONE + a*DVec3::Z);

    (255.*c).as_u8vec4()
}

fn main() {
    let cli = Cli::parse();

    // Image
    let (image_width, image_height, aspect_ratio) = cli.image_size.validate();

    // Camera
    let camera_center = DVec3::ZERO;
    let focal_length = cli.focal_length;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio*viewport_height;

    let viewport_u =  DVec3::X*viewport_width;
    let viewport_v = -DVec3::Y*viewport_height;

    let viewport_pixel_delta_u = viewport_u/(image_width as f64);
    let viewport_pixel_delta_v = viewport_v/(image_height as f64);

    let viewport_top_left =
            camera_center
                - DVec3::Z*focal_length
                - viewport_u/2.
                - viewport_v/2.
                + (viewport_pixel_delta_u + viewport_pixel_delta_v)/2.
            ;

    // Render
    let pixels: Vec<U8Vec4> =
        Itertools::cartesian_product(0..image_height, 0..image_width)
            .map(|(y, x)| {
                let pixel =
                    viewport_top_left
                        + (x as f64)*viewport_pixel_delta_u
                        + (y as f64)*viewport_pixel_delta_v
                    ;

                let direction = pixel - camera_center;
                let ray = Ray::new(camera_center, direction);

                ray_color(&ray)
            })
            .collect();

    let mut ppm_out = File::options()
        .create(true)
        .create_new(if cli.force_overwrite { false } else { true })
        .truncate(if cli.force_overwrite { true } else { false })
        .write(true)
        .open(cli.output.unwrap_or("out.ppm".try_into().unwrap()))
        .expect("Fail to open {} for writing");

    write_ppm(&mut ppm_out, image_width, image_height, pixels.as_slice())
        .expect("Fail to write ppm");
}
