use crate::core::dir;
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

static CONFIG_FILENAME: &str = "config.yml";

lazy_static! {
    pub static ref CONFIG: Config = Config::load().expect("failed to load the hocus config");
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub analytics_enabled: bool,
}

impl Config {
    fn load() -> Result<Config> {
        let app_dir = dir::get_app_dir()?;
        let config_path = app_dir.join(CONFIG_FILENAME);
        if config_path.exists() {
            let config_str = fs::read_to_string(config_path)?;
            serde_yaml::from_str(&config_str).context("failed to parse the Hocus config file")
        } else {
            let config = Config {
                analytics_enabled: true,
            };

            let config_str = serde_yaml::to_string(&config)?;
            fs::write(config_path, config_str).context("failed to save the Hocus config file")?;

            Ok(config)
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub mount_service: String,
    pub mount_dir: String,
    pub script_workdirs: ScriptWorkdirs,
    pub script_paths: ScriptWorkdirs,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptWorkdirs {
    pub init: String,
    pub shell: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptPaths {
    pub init: String,
    pub shell: String,
}

impl ProjectConfig {
    pub fn open(project_dir: &Path) -> Result<ProjectConfig> {
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
}
