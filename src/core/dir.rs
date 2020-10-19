use anyhow::{anyhow, Result};
use std::path::PathBuf;

const HOCUS_DIR_NAME: &str = ".hocus";

pub fn get_app_dir() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(anyhow!("failed to get the path to the home directory"))?;
    Ok(dir.join(HOCUS_DIR_NAME))
}
