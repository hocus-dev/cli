use super::Action;
use crate::cmd::edit::EditCmd;
use crate::core::dir::get_project_dir;
use crate::core::shell::run_command;
use anyhow::Result;
use std::process::Command;

impl Action for EditCmd {
    fn run(&self) -> Result<()> {
        let project_dir = get_project_dir(&self.name)?;
        println!("Opening {} in VSCode...", project_dir.display());
        run_command(Command::new("code").arg(project_dir))?;

        Ok(())
    }
}
