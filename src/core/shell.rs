use anyhow::{anyhow, Result};
use std::io;
use std::io::Write;
use std::process::Command;

pub fn run_command(cmd: &mut Command) -> Result<()> {
    let output = cmd.output()?;
    if !output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        return Err(anyhow!("failed to execute a shell command"));
    }
    Ok(())
}
