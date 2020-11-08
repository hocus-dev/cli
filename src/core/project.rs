use crate::core::dir::get_project_dir;
use crate::core::shell::{get_first_line, run_command_and_get_output};
use anyhow::{anyhow, Context, Result};
use std::process::Command;

pub fn get_project_docker_prefix(project_name: &str) -> String {
    format!("hocus_{}", project_name)
}

pub fn get_docker_container_name(project_name: &str, service_name: &str) -> Result<String> {
    let project_dir = get_project_dir(project_name)?;
    let project_prefix = get_project_docker_prefix(project_name);

    let output = run_command_and_get_output(
        Command::new("docker-compose")
            .current_dir(&project_dir)
            .args(&["-p", &project_prefix, "ps", "-q", service_name]),
    )
    .context("failed to query for container id")?;
    let stdout = String::from_utf8(output.stdout)?;
    let container_id =
        get_first_line(&stdout).ok_or(anyhow!("failed to find a container id in '{}'", &stdout))?;

    let output = run_command_and_get_output(Command::new("docker").args(&[
        "inspect",
        "--format",
        "{{.Name}}",
        container_id,
    ]))
    .context("failed to query for container id")?;
    let stdout = String::from_utf8(output.stdout)?;
    let container_name = get_first_line(&stdout)
        .ok_or(anyhow!("failed to find a container name in '{}'", &stdout))?;

    Ok(container_name.to_string())
}
