use crate::utils::Language;
use anyhow::Result;
use inquire::{ui::RenderConfig, Select};
use strum::IntoEnumIterator;

pub fn prompt(rcfg: &RenderConfig) -> Result<Language> {
    let ans = Select::new(
        "Which language would you like to use?",
        Language::iter().collect(),
    )
    .with_render_config(*rcfg)
    .prompt()?;
    Ok(ans)
}
