use self::location::{location_validator, Location};
use anyhow::Result;
use inquire::ui::{Attributes, RenderConfig, StyleSheet};

mod location;

pub struct PromptRes {
    pub location: Location,
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

    Ok(PromptRes { location })
}
