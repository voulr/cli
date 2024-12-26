use crate::{
    create::{create, CreateArgs},
    init::init,
};
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
    Create(CreateArgs),
    Init,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Create(args) => create(args)?,
        Commands::Init => init()?,
    };

    Ok(())
}
