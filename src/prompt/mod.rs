use self::location::{location_validator, Location};
use crate::utils::{Framework, Language};
use anyhow::Result;
use inquire::ui::{Attributes, RenderConfig, StyleSheet};

mod framework;
mod language;
mod location;

pub struct PromptRes {
    pub location: Location,
    pub language: Language,
    pub framework: Framework,
}

pub fn prompt(location: &Option<String>) -> Result<PromptRes> {
    let rcfg = RenderConfig {
        prompt: StyleSheet {
            att: Attributes::BOLD,
            ..Default::default()
        },
        ..Default::default()
    };

    let location = match location {
        Some(loc) => location_validator(loc)?,
        None => location::prompt(&rcfg)?,
    };
    let language = language::prompt(&rcfg)?;
    let framework = framework::prompt(&rcfg, &language)?;

    Ok(PromptRes {
        location,
        language,
        framework,
    })
}
