use crate::prompt::Input;
use anyhow::Result;

pub fn create(input: Input) -> Result<()> {
    println!("creating project at: {:?}", &input.location);
    Ok(())
}
