use crate::prompt::prompt;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct CreateArgs {
    #[arg(value_name = "LOCATION")]
    location: Option<String>,
}

pub fn create(args: CreateArgs) -> Result<()> {
    let input = prompt()?;
    println!("creating project at: {}", &input.location.name);
    Ok(())
}
