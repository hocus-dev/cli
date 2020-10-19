use crate::cmd::{Root, SubCommand};
use anyhow::Result;

mod init;
mod new;

trait Action {
    fn run(&self) -> Result<()>;
}

pub fn invoke_action(root: Root) -> Result<()> {
    match root.subcmd {
        SubCommand::Init(args) => args.run(),
        SubCommand::New(args) => args.run(),
    }
}
