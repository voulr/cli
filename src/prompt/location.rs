use crate::utils::DEFAULT_LOCATION;
use anyhow::{bail, Result};
use inquire::{ui::RenderConfig, validator::Validation, Text};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub path: PathBuf,
}

pub fn prompt(rcfg: &RenderConfig) -> Result<Location> {
    let input = Text::new("Where would you like your project to be created?")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
        .with_validator(|text: &str| match get_location(text) {
            Ok(location) => {
                if !location.name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
                {
                    Ok(Validation::Invalid(
					    format!("{} must only contain lowercase alphanumeric characters, dashes, and underscores", location.name).into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            }
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?;

    let location = get_location(&input)?;
    Ok(location)
}

fn get_location(input: &str) -> Result<Location> {
    let path = PathBuf::from(input.trim());
    let name = path
        .file_name()
        .unwrap_or(OsStr::new(&path))
        .to_string_lossy()
        .to_string();

    if path.is_file() {
        bail!("{} is a file that exists", name);
    }
    if path.is_dir() && path.read_dir()?.next().is_some() {
        bail!("{} is a directory that is not empty", name);
    }

    Ok(Location { name, path })
}
