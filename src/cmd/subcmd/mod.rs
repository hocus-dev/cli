pub mod init;

pub enum Subcmd {
    Init,
}

impl Subcmd {
    /// Returns the name that a subcommand uses in the CLI.
    pub fn name(&self) -> &'static str {
        match self {
            Subcmd::Init => "init",
        }
    }

    pub fn from_name(name: &str) -> Option<Subcmd> {
        match name {
            _ if name == Subcmd::Init.name() => Some(Subcmd::Init),
            _ => None,
        }
    }
}

// Re-export to cmd module
pub use self::init::CmdInit;
