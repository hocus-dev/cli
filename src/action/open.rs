use super::Action;
use crate::cmd::open::OpenCmd;
use crate::core::dir::get_app_dir;
use crate::core::shell::run_command;
use anyhow::Result;
use std::process::Command;

impl Action for OpenCmd {
    fn run(&self) -> Result<()> {
        let hocus_dir = get_app_dir()?;
        let project_dir = hocus_dir.join(&self.name);

        println!("Starting the Docker environment...");
        run_command(
            Command::new("docker-compose")
                .current_dir(&project_dir)
                .arg("-p")
                .arg(format!("hocus_{}", &self.name))
                .arg("up")
                .arg("-d"),
        )?;

        println!("Opening the project in VSCode...");
        let container_json = format!("{{\"containerName\":\"/hocus_{}_app_1\"}}", &self.name);
        let uri = format!(
            "vscode-remote://attached-container+{}/home/hocus/code/",
            hex::encode(container_json)
        );
        run_command(Command::new("code").arg("--folder-uri").arg(uri))?;

        Ok(())
    }
}
