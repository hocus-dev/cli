use super::Action;
use crate::cmd::open::OpenCmd;
use crate::core::config::read_project_config;
use crate::core::dir::get_app_dir;
use crate::core::shell::run_command;
use anyhow::Result;
use std::process::Command;

impl Action for OpenCmd {
    fn run(&self) -> Result<()> {
        let hocus_dir = get_app_dir()?;
        let project_dir = hocus_dir.join(&self.name);

        let project_config = read_project_config(&project_dir)?;

        println!("Starting the Docker environment...");
        run_command(
            Command::new("docker-compose")
                .current_dir(&project_dir)
                .arg("-p")
                .arg(format!("hocus_{}", &self.name))
                .arg("up")
                .arg("-d"),
        )?;

        let container_name = format!(
            "hocus_{project}_{service}_1",
            project = &self.name,
            service = project_config.mount_service
        );

        run_command(
            Command::new("docker")
                .current_dir(&project_dir)
                .arg("cp")
                .arg("vscode-settings.json")
                .arg(format!(
                    "{container_name}:{path}",
                    container_name = &container_name,
                    path = "/home/hocus/.vscode-server/data/Machine/settings.json"
                )),
        )?;

        println!("Opening the project in VSCode...");
        let container_json = format!(
            "{{\"containerName\":\"/{container_name}\"}}",
            container_name = &container_name,
        );
        let uri = format!(
            "vscode-remote://attached-container+{container}{dir}",
            container = hex::encode(container_json),
            dir = project_config.mount_dir,
        );
        run_command(Command::new("code").arg("--folder-uri").arg(uri))?;

        Ok(())
    }
}
