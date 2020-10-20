use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub mount_service: String,
    pub mount_dir: String,
    pub script_workdirs: ScriptWorkdirs,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptWorkdirs {
    pub init: String,
    pub shell: String,
}

pub fn read_project_config(project_dir: &Path) -> Result<ProjectConfig> {
    let possible_names = ["hocus.yml", "hocus.yaml"];
    let config_path = {
        let mut maybe_path = None;
        for name in &possible_names {
            let path = project_dir.join(name);
            if path.exists() {
                maybe_path = Some(path);
                break;
            }
        }
        maybe_path
    }
    .ok_or(anyhow!("project config file does not exist"))?;

    let config_str = fs::read_to_string(config_path)?;

    serde_yaml::from_str(&config_str).context("failed to parse the project config file")
}
