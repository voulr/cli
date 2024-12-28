use crate::{create::create, init::init, prompt::prompt, utils::DEFAULT_LOCATION};
use anyhow::Result;
use clap::{Parser, Subcommand};

mod create;
mod init;
mod prompt;
mod utils;

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
    #[command(about = "Sets up a new Voulr project in the current directory")]
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name } => create(prompt(&name)?)?,
        Commands::Init => {
            let name = Some(DEFAULT_LOCATION.to_string());
            init(prompt(&name)?)?
        }
    };

    Ok(())
}
