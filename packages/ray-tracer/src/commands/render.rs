use std::fs;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{
    anyhow,
    Result,
};

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

impl SceneConfig {
    pub fn try_make_scene(
        self,
        args: &Render,
    ) -> Result<Scene> {
        let mut textures = Vec::<Arc::<dyn Texture + Send + Sync>>::new();
        for texture_config in self.textures {
            textures.push(
                texture_config.try_make_texture(textures.as_slice())?
            );
        }

        let mut materials = Vec::<Arc::<dyn Material + Send + Sync>>::new();
        for material_config in self.materials {
            materials.push(
                material_config.try_make_material(textures.as_slice())?
            );
        }

        let mut object_configs = self.objects;
        let mut objects = Vec::<Arc::<dyn Hitable + Send + Sync>>::new();
        while let Some(object) = ObjectConfig::try_make_object(
            &mut object_configs,
            &materials,
        )? {
            objects.push(object);
        }

        let mut camera_builder = CameraBuilder::default();

        self.camera.try_update(&mut camera_builder)?;
        args.camera.try_update(&mut camera_builder)?;

        Ok(Scene {
            camera: camera_builder.build(),
            objects: BVH::from(objects.as_mut_slice()),
        })
    }

    pub fn try_load_scene(args: &Render) -> Result<Scene> {
        let s = fs::read_to_string(&args.scene)?;

        let config = match args.scene.extension().and_then(OsStr::to_str) {
            Some("json") => serde_json::from_str::<SceneConfig>(&s)?,
            Some("toml") => toml::from_str::<SceneConfig>(&s)?,
            _ => {
                return Err(anyhow!("invalid scene file format!"));
            }
        };

        config.try_make_scene(args)
    }
}

pub fn run(args: &Render) -> Result<()> {
    let (mut file, format) = args.image.get_file()?;

    let scene = SceneConfig::try_load_scene(&args)?;
    let image = render_scene(args, &scene);

    dump_image(args, &mut file, image, format)
}
