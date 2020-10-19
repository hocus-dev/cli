use super::Action;
use crate::cmd::init::InitCmd;
use crate::core::dir::get_app_dir;
use anyhow::{Context, Result};
use std::fs::create_dir;

impl Action for InitCmd {
    fn run(&self) -> Result<()> {
        let hocus_dir = get_app_dir()?;
        create_dir(&hocus_dir).context("failed to create the hocus directory")?;
        println!("Initialized hocus successfully in {:?}", hocus_dir);

        Ok(())
    }
}
