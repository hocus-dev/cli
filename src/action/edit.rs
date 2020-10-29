use super::Action;
use crate::cmd::edit::EditCmd;
use crate::core::dir::get_project_dir;
use crate::core::shell::{code_command, run_command};
use anyhow::Result;

impl Action for EditCmd {
    fn run(&self) -> Result<()> {
        let project_dir = get_project_dir(&self.name)?;
        println!("Opening {} in VSCode...", project_dir.display());
        run_command(code_command().arg("--folder-uri").arg(project_dir))?;

        Ok(())
    }
}
