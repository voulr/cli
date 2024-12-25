use anyhow::Result;
use clap::{Parser, Subcommand};

mod create;
mod init;
mod input;

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
        Commands::Create => create::create(input::prompt()?),
        Commands::Init => init::init(input::prompt()?),
    };

    Ok(())
}
