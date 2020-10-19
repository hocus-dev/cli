use super::Action;
use crate::core::dir::get_app_dir;
use crate::core::shell::run_command;
use anyhow::{Context, Result};
use std::fs::{create_dir, remove_dir_all, rename};
use std::path::Path;
use std::process::Command;
use url::Url;

const DEFAULT_REPO: &str = "https://github.com/hocus-dev/templates";
const DEFAULT_SUBDIR: &str = "base";

pub struct NewAction<'a> {
    pub project_name: &'a str,
}

impl Action for NewAction<'_> {
    fn run(&self) -> Result<()> {
        let app_dir = get_app_dir()?;
        let project_dir = app_dir.join(self.project_name);
        let tmp_dir = app_dir.join(format!("tmp-{}", self.project_name));

        let result = setup_project(&project_dir, &tmp_dir, DEFAULT_REPO, DEFAULT_SUBDIR);
        remove_dir_all(tmp_dir).unwrap_or(());
        if let Err(err) = result {
            remove_dir_all(project_dir).unwrap_or(());
            return Err(err);
        }

        Ok(())
    }
}

fn setup_project(project_dir: &Path, tmp_dir: &Path, repo: &str, subdir: &str) -> Result<()> {
    create_dir(&tmp_dir).context(format!(
        "failed to create the temporary templates directory in {}",
        tmp_dir.display()
    ))?;

    let url = Url::parse(repo).context("repository address is not a valid URL")?;

    println!("Downloading the project template...");
    run_command(
        Command::new("git")
            .arg("-C")
            .arg(format!("{}", tmp_dir.display()))
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(format!("{}", url))
            .arg("."),
    )?;

    rename(tmp_dir.join(subdir), project_dir)
        .context("Failed to move the project template to the project directory.")?;

    println!(
        "Initialized the project successfully in {}",
        project_dir.display()
    );

    Ok(())
}
