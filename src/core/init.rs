use crate::core::dir::get_app_dir;
use anyhow::{Context, Result};
use std::fs::create_dir;

/// Initializes hocus on this machine. If it has been done already, does nothing.
pub fn init() -> Result<()> {
    let hocus_dir = get_app_dir()?;
    if !hocus_dir.exists() {
        create_dir(&hocus_dir).context("failed to create the hocus directory")?;
    }
    Ok(())
}
