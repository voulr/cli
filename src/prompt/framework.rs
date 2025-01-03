use crate::prompt::language::Language;
use anyhow::Result;
use inquire::{ui::RenderConfig, Select};
use strum::{Display, EnumIter};

#[derive(EnumIter, Display)]
pub enum Framework {
    Axum,
}

pub fn prompt(rcfg: &RenderConfig, lang: &Language) -> Result<Framework> {
    let ans = Select::new("Which framework would you like to use?", lang.frameworks())
        .with_render_config(*rcfg)
        .prompt()?;
    Ok(ans)
}
