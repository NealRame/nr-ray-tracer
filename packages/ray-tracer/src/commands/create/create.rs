use std::path::PathBuf;

use anyhow::Result;

use clap::{
    Args,
    Parser,
    Subcommand,
    ValueEnum,
};

use crate::cli::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(super) enum SceneConfigFormat {
    Json,
    Toml,
}

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct CreateArgs {
    #[command(flatten)]
    pub camera: CameraConfig,

    /// Force output overwrite.
    #[arg(short = 'f', long)]
    pub force_overwrite: bool,

    /// Specify the output format of the scene configuration
    #[arg(short = 'F', long, default_value = "toml")]
    pub format: SceneConfigFormat,

    /// Output file path.
    #[arg(short = 'o', long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct ConvertSTLArgs {
    #[command(flatten)]
    pub camera: CameraConfig,

    /// Force output overwrite.
    #[arg(short = 'f', long)]
    pub force_overwrite: bool,

    /// Specify the output format of the scene configuration
    #[arg(short = 'F', long, default_value = "toml")]
    pub format: SceneConfigFormat,

    /// Output file path.
    #[arg(short = 'o', long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// STL input file
    pub stl_file: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Render cornell box scene
    CornellBox(CreateArgs),

    /// Render earth scene
    Earth(CreateArgs),

    /// Render noise scene
    Noise(CreateArgs),

    /// Render five quads scene
    Quads(CreateArgs),

    /// Render five triangles scene
    Triangles(CreateArgs),

    /// Render sphere1 scene
    Sphere(CreateArgs),

    /// Render simple-lights scene
    SimpleLights(CreateArgs),

    /// Convert STL file
    ConvertSTL(ConvertSTLArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Create {
    #[command(subcommand)]
    command: Commands,
}

pub fn run(create: &Create) -> Result<()> {
    match &create.command {
        Commands::CornellBox(args) => super::cornell_box::run(args)?,
        Commands::Earth(args) => super::earth::run(args)?,
        Commands::Noise(args) => super::noise::run(args)?,
        Commands::Quads(args) => super::quads::run(args)?,
        Commands::Triangles(args) => super::triangles::run(args)?,
        Commands::Sphere(args) => super::spheres::run(args)?,
        Commands::SimpleLights(args) => super::simple_lights::run(args)?,
        Commands::ConvertSTL(args) => super::convert_stl::run(args)?,
    }
    Ok(())
}
