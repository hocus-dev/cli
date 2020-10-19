use super::Subcmd;
use clap::{App, SubCommand};

/// The init command definition.
pub struct CmdInit;

impl CmdInit {
    pub fn build<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Subcmd::Init.name()).about("Initializes hocus on this machine")
    }
}
