use crate::cmd::{Handler, Subcmd};
use anyhow::{anyhow, Result};

mod init;
mod new;

pub fn invoke_action(handler: Handler) -> Result<()> {
    let subcmd_name = handler
        .matches
        .subcommand_name()
        .ok_or(anyhow!("no subcommand matched"))?;
    let subcmd = Subcmd::from_name(subcmd_name).ok_or(anyhow!("no subcommand matched"))?;

    subcmd_to_action(subcmd, handler)
}

fn subcmd_to_action(subcmd: Subcmd, _handler: Handler) -> Result<()> {
    match subcmd {
        Subcmd::Init => (init::InitAction {}).run(),
    }
}

trait Action {
    fn run(&self) -> Result<()>;
}
