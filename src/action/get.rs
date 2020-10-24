use super::Action;
use crate::cmd::get::GetCmd;
use crate::core::dir::get_app_dir;
use crate::core::shell::run_command_with_stream_inheritance;
use anyhow::Result;
use std::process::Command;

impl Action for GetCmd {
    fn run(&self) -> Result<()> {
        let app_dir = get_app_dir()?;

        run_command_with_stream_inheritance(
            Command::new("git")
                .current_dir(&app_dir)
                .arg("clone")
                .arg(&self.url)
                .arg(&self.name),
        )?;

        println!(
            "The environment is now available in {}",
            app_dir.join(&self.name).display()
        );

        Ok(())
    }
}
