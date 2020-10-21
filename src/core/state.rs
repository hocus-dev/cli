use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILENAME: &str = ".hocus-state.yml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectState {
    pub is_init: bool,
}

impl ProjectState {
    pub fn open(project_dir: &Path) -> Result<ProjectState> {
        let state_path = project_dir.join(FILENAME);
        if state_path.exists() {
            let state_str = fs::read_to_string(state_path)?;
            serde_yaml::from_str(&state_str).context("failed to parse the project state file")
        } else {
            Ok(ProjectState { is_init: false })
        }
    }

    pub fn save(&self, project_dir: &Path) -> Result<()> {
        let state_path = project_dir.join(FILENAME);
        let state_str = serde_yaml::to_string(self)?;
        fs::write(state_path, state_str).context("failed to write the project state file")
    }
}
