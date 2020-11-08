use anyhow::{anyhow, Result};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::{Command, Output};

fn is_running_on_wsl() -> bool {
    let check_os_version = || -> Result<bool> {
        let mut os_version = String::new();
        File::open("/proc/version")?.read_to_string(&mut os_version)?;
        Ok(os_version.to_lowercase().contains("microsoft"))
    };
    // we ignore errors because on macOS /proc/version does not exist, so an error will be thrown
    match check_os_version() {
        Ok(result) => result,
        Err(_) => false,
    }
}

pub fn native_code_command() -> Command {
    if is_running_on_wsl() {
        // the in-WSL code command does not work like the ones on native Windows, Linux, or macOS,
        // so we use the windows-native one
        let mut command = Command::new("powershell.exe");
        command.arg("code");
        command
    } else {
        Command::new("code")
    }
}

pub fn run_command(cmd: &mut Command) -> Result<()> {
    if cmd.status()?.success() {
        Ok(())
    } else {
        Err(anyhow!("failed to execute a shell command"))
    }
}

pub fn run_command_and_get_output(cmd: &mut Command) -> Result<Output> {
    let output = cmd.output()?;
    if output.status.success() {
        Ok(output)
    } else {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Err(anyhow!("failed to execute a shell command"))
    }
}

pub fn get_first_line<'a>(text: &'a str) -> Option<&'a str> {
    text.lines().find(|l| l.len() > 0)
}
