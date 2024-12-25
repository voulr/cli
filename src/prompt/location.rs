use crate::utils::DEFAULT_LOCATION;
use anyhow::{bail, Result};
use inquire::{ui::RenderConfig, validator::Validation, Text};
use std::{ffi::OsStr, path::PathBuf};

pub fn prompt(rcfg: &RenderConfig) -> Result<PathBuf> {
    let location = Text::new("Where would you like your project to be created?")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
        .with_validator(|text: &str| match get_path(text) {
            Ok(path) => {
                let file_name = get_file_name(&path);
                if !file_name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
                {
                    Ok(Validation::Invalid(
					    format!("{} must only contain lowercase alphanumeric characters, dashes, and underscores", file_name).into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            }
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?;

    let path = get_path(&location)?;
    Ok(path)
}

fn get_path(input: &str) -> Result<PathBuf> {
    let path = PathBuf::from(input.trim());
    let file_name = get_file_name(&path);

    if path.is_file() {
        bail!("{} is a file that exists", file_name);
    }
    if path.is_dir() && path.read_dir()?.next().is_some() {
        bail!("{} is a directory that is not empty", file_name);
    }

    Ok(path)
}

fn get_file_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap_or(OsStr::new(path))
        .to_string_lossy()
        .to_string()
}
