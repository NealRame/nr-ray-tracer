use std::fs;
use std::io;
use std::path::{
    Path,
    PathBuf,
};
use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};

use anyhow::Result;

use clap::{
    Args,
    Parser,
    Subcommand,
    ValueEnum,
};

use crate::cli::*;

pub fn get_output<P: AsRef<Path>>(
    path: Option<P>,
    overwrite: bool,
) -> Result<Box<dyn io::Write>> {
    let output: Box<dyn io::Write> =
        if let Some(path) = path {
            Box::new(fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .create_new(!overwrite)
                .write(true)
                .open(path)?
            )
        } else {
            Box::new(io::stdout())
        };
    Ok(output)
}

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

    /// Seed for random generations.
    #[arg(short = 's', long, value_name = "SEED", default_value="1")]
    pub seed: u64,
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
    Spheres(CreateArgs),

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
        Commands::Spheres(args) => super::spheres::run(args)?,
        Commands::SimpleLights(args) => super::simple_lights::run(args)?,
        Commands::ConvertSTL(args) => super::convert_stl::run(args)?,
    }
    Ok(())
}

pub fn get_next_texture_id() -> Box<str> {
    static TEX_ID: AtomicUsize = AtomicUsize::new(0);

    let tex_id = TEX_ID.fetch_add(1, Ordering::Relaxed);
    format!("tex_{:07}", tex_id).into_boxed_str()
}

pub fn get_next_material_id() -> Box<str> {
    static MAT_ID: AtomicUsize = AtomicUsize::new(0);

    let mat_id = MAT_ID.fetch_add(1, Ordering::Relaxed);
    format!("mat_{:07}", mat_id).into_boxed_str()
}
