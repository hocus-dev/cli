use crate::cmd::{Root, SubCommand};
use anyhow::Result;

mod analytics;
mod close;
mod delete;
mod edit;
mod get;
mod open;

trait Action {
    fn run(&self) -> Result<()>;
}

pub fn invoke_action(root: Root) -> Result<()> {
    match root.subcmd {
        SubCommand::Edit(args) => args.run(),
        SubCommand::Open(args) => args.run(),
        SubCommand::Close(args) => args.run(),
        SubCommand::Get(args) => args.run(),
        SubCommand::Delete(args) => args.run(),
        SubCommand::Analytics(args) => args.run(),
    }
}
