use anyhow::{bail, Result};
use inquire::{ui::RenderConfig, validator::Validation, Text};
use std::{ffi::OsStr, path::PathBuf};

pub const DEFAULT_LOCATION: &str = "./";

#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub path: PathBuf,
}

impl Location {
    pub fn new(input: &str) -> Self {
        let path = PathBuf::from(input.trim());
        let name = path
            .file_name()
            .unwrap_or(OsStr::new(&path))
            .to_string_lossy()
            .to_string();
        Self { name, path }
    }

    pub fn validate(self) -> Result<Self> {
        if self.path.is_file() {
            bail!("{} is a file that exists", self.name);
        }
        Ok(self)
    }
}

pub fn prompt(rcfg: &RenderConfig) -> Result<Location> {
    let input = Text::new("Where would you like your project to be created?")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
        .with_validator(|text: &str| match Location::new(text).validate() {
            Ok(_) => Ok(Validation::Valid),
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?;

    let location = Location::new(&input).validate()?;
    Ok(location)
}
