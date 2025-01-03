use crate::prompt::framework::Framework;
use anyhow::Result;
use inquire::{ui::RenderConfig, Select};
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

#[derive(EnumIter, Display)]
pub enum Language {
    Rust,
}

impl Language {
    pub fn frameworks(&self) -> Vec<Framework> {
        match self {
            Language::Rust => vec![Framework::Axum],
        }
    }
}

pub fn prompt(rcfg: &RenderConfig) -> Result<Language> {
    let ans = Select::new(
        "Which language would you like to use?",
        Language::iter().collect(),
    )
    .with_render_config(*rcfg)
    .prompt()?;
    Ok(ans)
}
