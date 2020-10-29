use crate::cmd::{reset::ResetSubCommand, Root, SubCommand};
use anyhow::Result;

mod close;
mod edit;
mod get;
mod open;
mod reset;

trait Action {
    fn run(&self) -> Result<()>;
}

pub fn invoke_action(root: Root) -> Result<()> {
    match root.subcmd {
        SubCommand::Edit(args) => args.run(),
        SubCommand::Open(args) => args.run(),
        SubCommand::Close(args) => args.run(),
        SubCommand::Get(args) => args.run(),
        SubCommand::Reset(reset_cmd) => match reset_cmd.subcmd {
            ResetSubCommand::Volumes(args) => args.run(),
        },
    }
}
