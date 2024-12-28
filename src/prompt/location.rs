use crate::utils::DEFAULT_LOCATION;
use anyhow::{bail, Result};
use inquire::{ui::RenderConfig, validator::Validation, Text};
use regex::Regex;
use std::{ffi::OsStr, path::PathBuf};

pub struct Location {
    pub name: String,
    pub path: PathBuf,
}

pub fn prompt(rcfg: &RenderConfig) -> Result<Location> {
    let input = Text::new("Where would you like your project to be created?")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
        .with_validator(|text: &str| match location_validator(text) {
            Ok(_) => Ok(Validation::Valid),
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?;

    let location = get_location(&input);
    Ok(location)
}

pub fn get_location(input: &str) -> Location {
    let path = PathBuf::from(input.trim());
    let name = path
        .file_name()
        .unwrap_or(OsStr::new(&path))
        .to_string_lossy()
        .to_string();
    Location { name, path }
}

pub fn location_validator(input: &str) -> Result<Location> {
    let loc = get_location(input);

    if loc.path.is_file() {
        bail!("{} is a file that exists", loc.name);
    }
    if loc.path.is_dir() && loc.path.read_dir()?.next().is_some() {
        bail!("{} is a directory that is not empty", loc.name);
    }

    let name_regex = Regex::new(r"^[a-z0-9_-]+$").unwrap();
    if !name_regex.is_match(&loc.name) {
        bail!(
            "Location ({}) only contain lowercase alphanumeric characters, dashes, and underscores",
            loc.name
        );
    }

    Ok(loc)
}
