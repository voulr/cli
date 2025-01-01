use crate::prompt::PromptRes;
use anyhow::Result;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates/"]
struct Templates;

pub fn scaffold(pres: &PromptRes) -> Result<()> {
    for file in Templates::iter() {
        println!("{}", file);
    }
    Ok(())
}
