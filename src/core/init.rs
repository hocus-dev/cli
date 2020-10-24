use crate::core::dir::get_app_dir;
use anyhow::{Context, Result};
use std::fs::create_dir;

/// Initializes hocus on this machine if it hasn't been done already. Will return an error only if
/// one occured while initializng the environment. If hocus was initialized previously, no error
/// will be returned.
pub fn init() -> Result<()> {
    let hocus_dir = get_app_dir()?;
    if !hocus_dir.exists() {
        create_dir(&hocus_dir).context("failed to create the hocus directory")?;
    }
    Ok(())
}
