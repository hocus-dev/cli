use super::Action;
use crate::core::dir::get_app_dir;
use anyhow::{Context, Result};
use std::fs::create_dir;

pub struct NewAction<'a> {
    project_name: &'a str,
}

impl Action for NewAction<'_> {
    fn run(&self) -> Result<()> {
        let project_dir = get_app_dir()?.join(self.project_name);
        create_dir(&project_dir).context("failed to create the project directory")?;

        println!("Initialized the project successfully in {:?}", project_dir);

        Ok(())
    }
}
