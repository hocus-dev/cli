use super::Action;
use crate::cmd::edit::EditCmd;
use crate::core::dir::get_app_dir;
use crate::core::shell::{code_command, run_command};
use anyhow::Result;

impl Action for EditCmd {
    fn run(&self) -> Result<()> {
        let hocus_dir = get_app_dir()?;
        let project_dir = hocus_dir.join(&self.name);
        println!("Opening {} in VSCode...", project_dir.display());
        run_command(
            code_command()
                .arg("--folder-uri")
                .arg(format!("{}", project_dir.display())),
        )?;

        Ok(())
    }
}
