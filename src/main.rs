use crate::{create::create, init::init, prompt::prompt};
use anyhow::Result;
use clap::{Parser, Subcommand};

mod create;
mod init;
mod prompt;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Init,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Create => create(prompt()?)?,
        Commands::Init => init(prompt()?)?,
    };

    Ok(())
}
