use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{
    anyhow,
    Result,
};

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
    pub scene: PathBuf,

    #[command(flatten)]
    pub image: ImageArgs,

    #[command(flatten)]
    pub camera: CameraArgs,

    /// Force output overwrite.
    #[arg(short = 'f', long)]
    force_overwrite: bool,

    /// Output file path.
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Show progress.
    #[arg(short, long)]
    pub verbose: bool
}

impl Args {
    pub fn get_file(&self) -> Result<(fs::File, ImageFormat)> {
        let overwrite = self.force_overwrite;
        let filepath =
            self.output
                .clone()
                .or_else(|| {
                    match (
                        self.scene.parent(),
                        self.scene.file_stem().and_then(OsStr::to_str),
                    ) {
                        (Some(parent), Some(name)) => {
                            Some(parent.join(format!("{name}.png")))
                        },
                        _ => None
                    }
                })
                .unwrap_or("out.bmp".try_into().unwrap());

        let format = ImageFormat::from_path(filepath.as_path())?;

        let file =
            fs::File::options()
                .create_new(!overwrite)
                .create(true)
                .truncate(true)
                .write(true)
                .open(filepath.as_path())?
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

fn load_scene(
    cli: &Args,
) -> Result<Scene> {
    let content = fs::read_to_string(&cli.scene)?;

    let scene_config_ext =
        cli.scene.extension()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_lowercase());

    let mut scene_config: SceneConfig =
        match scene_config_ext.as_ref().map(|s| s.as_str()) {
            Some("json") => serde_json::from_str(&content)?,
            Some("toml") => toml::from_str(&content)?,
            _ => {
                return Err(anyhow!(
                    "Unsupported scene file format '{}'.",
                    cli.scene.to_string_lossy())
                );
            },
        };

    cli.camera.try_update(&mut scene_config.camera)?;

    println!("{:?}", cli.camera);
    println!("---",);
    println!("{:?}", scene_config.camera);

    Ok(Scene::from(scene_config))
}

fn render_scene(
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

fn dump_image(
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

pub fn run(args: &Args) -> Result<()> {
    let (mut file, format) = args.get_file()?;

    let scene = load_scene(&args)?;
    let image = render_scene(&args, &scene);

    dump_image(&args, &mut file, image, format)
}
