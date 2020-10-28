use super::Action;
use crate::cmd::get::GetCmd;
use crate::core::dir::{get_app_dir, get_generated_dir, get_project_dir};
use crate::core::shell::run_command;
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

impl Action for GetCmd {
    fn run(&self) -> Result<()> {
        let app_dir = get_app_dir()?;

        run_command(
            Command::new("git")
                .current_dir(&app_dir)
                .arg("clone")
                .arg(&self.url)
                .arg(&self.name),
        )?;

        let project_dir = get_project_dir(&self.name)?;

        fs::create_dir_all(get_generated_dir(&project_dir))
            .context("failed to create the generated directory")?;

        println!(
            "\nYou can now open the environment with:\n$ hocus open {}\n",
            &self.name
        );

        Ok(())
    }
}
