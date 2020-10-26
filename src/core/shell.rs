use anyhow::{anyhow, Result};
use std::process::Command;

pub fn run_command(cmd: &mut Command) -> Result<()> {
    if cmd.status()?.success() {
        Ok(())
    } else {
        Err(anyhow!("failed to execute a shell command"))
    }
}
