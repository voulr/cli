use crate::prompt::PromptRes;
use anyhow::Result;

pub fn init(pres: PromptRes) -> Result<()> {
    println!("init project at: {}", &pres.location.name);
    println!("using language: {}", &pres.language);
    println!("using framework: {}", &pres.framework);
    Ok(())
}
