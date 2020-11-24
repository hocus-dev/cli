use crate::cmd::{Root, SubCommand};
use crate::core::analytics::spawn_analytics_process;
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
    // we don't want a fork bomb, don't we?
    match root.subcmd {
        SubCommand::Analytics(_) => (),
        _ => spawn_analytics_process(),
    }

    match root.subcmd {
        SubCommand::Edit(args) => args.run(),
        SubCommand::Open(args) => args.run(),
        SubCommand::Close(args) => args.run(),
        SubCommand::Get(args) => args.run(),
        SubCommand::Delete(args) => args.run(),
        SubCommand::Analytics(args) => args.run(),
    }
}
