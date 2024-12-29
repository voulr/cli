use crate::prompt::PromptRes;
use anyhow::Result;
use scaffold::scaffold;

mod scaffold;

pub fn create(pres: PromptRes) -> Result<()> {
    scaffold(&pres);
    Ok(())
}
