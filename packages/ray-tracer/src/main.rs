
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
    Create(commands::create::Args),

    /// Render a given scene
    Render(commands::render::Args),
}

#[derive(Parser)]
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
