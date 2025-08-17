mod cli;
mod commands;
mod constants;

use anyhow::Result;

use clap::{
    Parser,
    Subcommand,
};

#[derive(Subcommand)]
enum Commands {
    /// Create a new scene
    // Create(commands::create::Args),

    /// Render cornell box scene
    CornellBox(commands::render::Args),

    /// Render earth scene
    Earth(commands::render::Args),

    /// Render simple lights scene
    SimpleLights(commands::render::Args),

    /// Render perlin scene
    Perlin(commands::render::Args),

    /// Render quads scene
    Quads(commands::render::Args),

    /// Render sphere1 scene
    Spheres1(commands::render::Args),

    /// Render sphere2 scene
    Spheres2(commands::render::Args),

    /// Render sphere3 scene
    Spheres3(commands::render::Args),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CornellBox(args) => commands::cornell_box::run(&args),
        Commands::Earth(args) => commands::earth::run(&args),
        Commands::SimpleLights(args) => commands::simple_lights::run(&args),
        Commands::Perlin(args) => commands::perlin::run(&args),
        Commands::Quads(args) => commands::quads::run(&args),
        Commands::Spheres1(args) => commands::spheres1::run(&args),
        Commands::Spheres2(args) => commands::spheres2::run(&args),
        Commands::Spheres3(args) => commands::spheres3::run(&args),
    }
}
