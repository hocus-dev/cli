use super::Subcmd;
use clap::{App, SubCommand};

/// The new command definition.
pub struct CmdNew;

impl CmdNew {
    pub fn build<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Subcmd::New.name()).about("Initializes a new project")
    }
}
