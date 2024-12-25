use std::{ffi::OsStr, path::PathBuf};

use crate::utils::DEFAULT_LOCATION;
use anyhow::{bail, Result};
use inquire::{ui::RenderConfig, Text};

pub fn prompt(rcfg: &RenderConfig) -> Result<PathBuf> {
    let location = Text::new("Where would you like your project to be created?")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
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
