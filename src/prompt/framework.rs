use crate::utils::{Framework, Language};
use anyhow::Result;
use inquire::{ui::RenderConfig, Select};

pub fn prompt(rcfg: &RenderConfig, lang: &Language) -> Result<Framework> {
    let ans = Select::new("Which framework would you like to use?", lang.frameworks())
        .with_render_config(*rcfg)
        .prompt()?;
    Ok(ans)
}
