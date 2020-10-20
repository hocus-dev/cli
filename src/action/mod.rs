use crate::cmd::{Root, SubCommand};
use anyhow::Result;

mod close;
mod edit;
mod init;
mod new;
mod open;

trait Action {
    fn run(&self) -> Result<()>;
}

pub fn invoke_action(root: Root) -> Result<()> {
    match root.subcmd {
        SubCommand::Init(args) => args.run(),
        SubCommand::New(args) => args.run(),
        SubCommand::Edit(args) => args.run(),
        SubCommand::Open(args) => args.run(),
        SubCommand::Close(args) => args.run(),
    }
}
