use super::Action;
use crate::core::dir::get_app_dir;
use anyhow::{Context, Result};
use std::fs::create_dir;

pub struct InitAction;

impl Action for InitAction {
    fn run(&self) -> Result<()> {
        let hocus_dir = get_app_dir()?;
        create_dir(&hocus_dir).context("failed to create the hocus directory")?;
        println!("Initialized hocus successfully in {:?}", hocus_dir);

        Ok(())
    }
}
