use self::location::Location;
use anyhow::Result;
use inquire::ui::{Attributes, RenderConfig, StyleSheet};

pub mod location;

#[derive(Debug)]
pub struct Input {
    pub location: Location,
}

pub fn prompt() -> Result<Input> {
    let rcfg = RenderConfig {
        prompt: StyleSheet {
            att: Attributes::BOLD,
            ..Default::default()
        },
        ..Default::default()
    };

    let location = location::prompt(&rcfg)?;

    Ok(Input { location })
}
