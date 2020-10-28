use super::Action;
use crate::cmd::close::CloseCmd;
use crate::core::dir::get_project_dir;
use crate::core::shell::run_command;
use anyhow::Result;
use std::process::Command;

impl Action for CloseCmd {
    fn run(&self) -> Result<()> {
        let project_dir = get_project_dir(&self.name)?;

        println!("Shutting down the Docker environment...");
        run_command(
            Command::new("docker-compose")
                .current_dir(&project_dir)
                .arg("-p")
                .arg(format!("hocus_{}", &self.name))
                .arg("down"),
        )?;

        println!("Done.");

        Ok(())
    }
}
