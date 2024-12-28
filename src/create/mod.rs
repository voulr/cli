use crate::prompt::PromptRes;
use anyhow::Result;

pub fn create(pres: PromptRes) -> Result<()> {
    println!("creating project at: {}", &pres.location.name);
    println!("using language: {}", &pres.language);
    println!("using framework: {}", &pres.framework);
    Ok(())
}
