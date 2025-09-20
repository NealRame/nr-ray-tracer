mod cli;
mod commands;
mod constants;
mod scene_config;

use anyhow::Result;

use clap::{
    Parser,
    Subcommand,
};

#[derive(Subcommand)]
enum Commands {
    /// Create a new scene
    Create(commands::create::Create),

    /// Render scene
    Render(commands::render::RenderArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create(args) => commands::create::run(&args),
        Commands::Render(args) => commands::render::run(&args),
    }
}
