use crate::{create::create, prompt::prompt};
use anyhow::Result;
use clap::{Parser, Subcommand};

mod create;
mod prompt;

#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Scaffolds new Voulr project", alias = "new")]
    Create { name: Option<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name } => create(prompt(&name)?)?,
    };

    Ok(())
}
