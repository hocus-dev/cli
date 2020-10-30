use super::Action;
use crate::cmd::delete::DeleteCmd;
use crate::core::dir::get_project_dir;
use crate::core::project::get_project_docker_prefix;
use crate::core::shell::{run_command, run_command_and_get_output};
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

impl Action for DeleteCmd {
    fn run(&self) -> Result<()> {
        let project_prefix = get_project_docker_prefix(&self.name);

        let volumes_stdout = String::from_utf8(
            run_command_and_get_output(Command::new("docker").args(&["volume", "ls", "-q"]))?
                .stdout,
        )?;

        let volumes: Vec<&str> = volumes_stdout
            .lines()
            .filter(|vol| vol.starts_with(&project_prefix))
            .collect();

        match volumes.len() {
            0 => println!("No volumes to delete."),
            _ => {
                println!("Deleting volumes:");
                run_command(Command::new("docker").arg("volume").arg("rm").args(volumes))?;
            }
        }

        let project_dir = get_project_dir(&self.name)?;
        println!(
            "Removing the project directory at {}",
            project_dir.display()
        );
        fs::remove_dir_all(project_dir).context("failed to delete the project directory")?;

        Ok(())
    }
}
