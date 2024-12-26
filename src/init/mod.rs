use crate::prompt::prompt;
use anyhow::Result;

pub fn init() -> Result<()> {
    let input = prompt()?;
    println!("init project at: {}", &input.location.name);
    Ok(())
}
