use std::collections::HashMap;
use std::fs;
use std::ffi::OsStr;
use std::path::{
    Path,
    PathBuf,
};
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

type TextureMap = HashMap<Box<str>, Arc<dyn Texture + Send + Sync>>;
type MaterialMap = HashMap<Box<str>, Arc<dyn Material + Send + Sync>>;
type InstanceMap = HashMap<Box<str>, Arc<dyn Hitable + Send + Sync>>;

impl SceneConfig {
    pub fn try_build(self) -> Result<Scene> {
        let mut textures = TextureMap::new();
        for (texture_id, texture_config) in self.textures {
            let texture = texture_config.try_make_texture(&textures)?;
            textures.insert(texture_id.clone(), texture);
        }

        let mut materials = MaterialMap::new();
        for (material_id, material_config) in self.materials {
            let material = material_config.try_make_material(&textures)?;
            materials.insert(material_id.clone(), material);
        }

        let mut instances = InstanceMap::new();
        for (instance_id, instance_config) in self.instances {
            let object = instance_config.try_make_object(
                &instances,
                &materials,
            )?;
            instances.insert(instance_id.clone(), object);
        }

        let mut objects = Vec::new();
        for object_config in self.scene {
            let object = object_config.try_make_object(
                &instances,
                &materials,
            )?;
            objects.push(object);
        }

        let mut camera_builder = CameraBuilder::default();

        self.camera.try_update(&mut camera_builder)?;

        let camera = camera_builder.build();

        Ok(Scene {
            camera,
            objects: BVH::from(objects.as_mut_slice()),
        })
    }

    pub fn try_load_scene<P: AsRef<Path>>(path: P) -> Result<Self> {
        let ext = path.as_ref().extension().and_then(OsStr::to_str);

        let scene_config = match ext {
            Some("json") => {
                let s = fs::read_to_string(path.as_ref())?;
                serde_json::from_str::<SceneConfig>(&s)?
            },
            Some("toml") => {
                let s = fs::read_to_string(path.as_ref())?;
                toml::from_str::<SceneConfig>(&s)?
            },
            _ => {
                return Err(anyhow!("invalid scene file format!"));
            }
        };

        Ok(scene_config)
    }
}

pub fn run(args: &Render) -> Result<()> {
    let (mut file, format) = args.image.get_file()?;

    let mut scene_config = SceneConfig::try_load_scene(args.scene.as_path())?;

    scene_config.camera.merge_with(&args.camera);

    let scene = scene_config.try_build()?;
    let image = render_scene(args, &scene);

    dump_image(args, &mut file, image, format)
}
