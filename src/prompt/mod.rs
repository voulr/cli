use self::location::{validate_location, Location};
use anyhow::Result;
use inquire::ui::{Attributes, RenderConfig, StyleSheet};

pub mod location;

pub struct PromptRes {
    pub location: Location,
}

pub fn prompt(loc_arg: &Option<String>) -> Result<PromptRes> {
    let rcfg = RenderConfig {
        prompt: StyleSheet {
            att: Attributes::BOLD,
            ..Default::default()
        },
        ..Default::default()
    };

    let location = match loc_arg {
        Some(loc) => validate_location(loc)?,
        None => location::prompt(&rcfg)?,
    };

    Ok(PromptRes { location })
}
