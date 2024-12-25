use anyhow::Result;
use inquire::{
    ui::{Attributes, RenderConfig, StyleSheet},
    Text,
};

#[derive(Debug)]
pub struct UserInput {
    pub name: String,
}

pub fn prompt() -> Result<UserInput> {
    let render_config = RenderConfig {
        prompt: StyleSheet {
            att: Attributes::BOLD,
            ..Default::default()
        },
        ..Default::default()
    };

    let name = Text::new("whats your name?")
        .with_render_config(render_config)
        .prompt()?;

    let res = UserInput { name };
    Ok(res)
}
