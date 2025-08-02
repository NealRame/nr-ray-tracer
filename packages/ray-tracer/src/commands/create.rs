use std::{fs, io::{stdout, Write}, path::PathBuf};

use anyhow::Result;

use clap::ValueEnum;

use nr_ray_tracer_lib::prelude::*;

use crate::cli::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum SceneConfigFormat {
    Json,
    Toml,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    image: ImageArgs,

    #[command(flatten)]
    camera: CameraArgs,

    /// Force output overwrite.
    #[arg(short = 'f', long)]
    force_overwrite: bool,

    /// Specify the output format of the scene configuration
    #[arg(short = 'F', long, default_value = "toml")]
    format: SceneConfigFormat,

    /// Output file path.
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<PathBuf>,
}

pub fn run(args: &Args) -> Result<()>{
    let mut scene_config = SceneConfig::default();

    args.camera.try_update(&mut scene_config.camera)?;

    let contents = match args.format {
        SceneConfigFormat::Json => serde_json::to_string_pretty(&scene_config)?,
        SceneConfigFormat::Toml => toml::to_string_pretty(&scene_config)?,
    };

    if let Some(output) = args.output.as_ref() {
        fs::write(output, &contents)?
    } else {
        stdout().write_all(contents.as_bytes())?;
    }

    Ok(())
}
