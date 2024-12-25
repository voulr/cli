use crate::input::UserInput;
use anyhow::Result;

pub fn init(input: UserInput) -> Result<()> {
    println!("init project with name: {}", &input.name);
    Ok(())
}
