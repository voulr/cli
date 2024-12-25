use crate::input::UserInput;
use anyhow::Result;

pub fn create(input: UserInput) -> Result<()> {
    println!("creating project with name: {}", &input.name);
    Ok(())
}
