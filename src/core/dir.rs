use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

const HOCUS_DIR_NAME: &str = ".hocus";
const GENERATED_DIR_NAME: &str = "user";

pub fn get_app_dir() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(anyhow!("failed to get the path to the home directory"))?;
    Ok(dir.join(HOCUS_DIR_NAME))
}

pub fn get_project_dir(project_name: &str) -> Result<PathBuf> {
    let hocus_dir = get_app_dir()?;
    Ok(hocus_dir.join(project_name))
}

pub fn get_generated_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(GENERATED_DIR_NAME)
}
