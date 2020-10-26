use super::Action;
use crate::cmd::open::OpenCmd;
use crate::core::config::ProjectConfig;
use crate::core::dir::get_app_dir;
use crate::core::shell::{code_command, run_command};
use crate::core::state::ProjectState;
use anyhow::Result;
use std::fs;
use std::process::Command;

impl Action for OpenCmd {
    fn run(&self) -> Result<()> {
        let hocus_dir = get_app_dir()?;
        let project_dir = hocus_dir.join(&self.name);

        let project_config = ProjectConfig::open(&project_dir)?;

        println!("Starting the Docker environment...");
        run_command(
            Command::new("docker-compose")
                .current_dir(&project_dir)
                .arg("-p")
                .arg(format!("hocus_{}", &self.name))
                .arg("up")
                .arg("--build")
                .arg("-d"),
        )?;

        let container_name = format!(
            "hocus_{project}_{service}_1",
            project = &self.name,
            service = project_config.mount_service
        );

        let mut project_state = ProjectState::open(&project_dir)?;
        if !project_state.is_init {
            println!("Creating the .env file using template.env...");
            fs::copy(project_dir.join("template.env"), project_dir.join(".env"))?;

            println!("Running the init.sh script...");
            run_command(
                Command::new("docker")
                    .arg("exec")
                    .arg("-it") // keep stdin open, allow user interaction
                    .arg(&container_name)
                    .arg("/bin/bash")
                    .arg("-c")
                    .arg("-e") // exit on error
                    .arg(format!(
                        "cd {workdir} && /bin/bash -e {init_script}",
                        workdir = project_config.script_workdirs.init,
                        init_script = project_config.script_paths.init,
                    )),
            )?;
            project_state.is_init = true;
            project_state.save(&project_dir)?;
        }

        println!("Opening the project in VSCode...");
        let container_json = format!(
            "{{\"containerName\":\"/{container_name}\"}}",
            container_name = &container_name,
        );
        // URI format taken from:
        // https://github.com/microsoft/vscode-remote-release/issues/2133
        let uri = format!(
            "vscode-remote://attached-container+{container}{dir}",
            container = hex::encode(container_json),
            dir = project_config.mount_dir,
        );
        run_command(code_command().arg("--folder-uri").arg(uri))?;

        Ok(())
    }
}
