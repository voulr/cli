use crate::prompt::PromptRes;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn scaffold(pres: &PromptRes) -> Result<()> {
    let in_dir = get_template_path(pres);
    let out_dir = pres.location.path.clone();

    fs::create_dir_all(&out_dir)?;

    for entry in WalkDir::new(&in_dir) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(&in_dir)?;
        let target = out_dir.join(relative);

        if path.is_dir() {
            fs::create_dir_all(target)?;
        } else {
            fs::copy(path, target)?;
        }
    }

    Ok(())
}

fn get_template_path(pres: &PromptRes) -> PathBuf {
    let s = format!("./templates/{}/{}/", pres.language, pres.framework).to_lowercase();
    PathBuf::from(s)
}
