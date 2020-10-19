pub mod init;
pub mod new;

pub enum Subcmd {
    Init,
    New,
}

impl Subcmd {
    /// Returns the name that a subcommand uses in the CLI.
    pub fn name(&self) -> &'static str {
        match self {
            Subcmd::Init => "init",
            Subcmd::New => "new",
        }
    }

    pub fn from_name(name: &str) -> Option<Subcmd> {
        match name {
            _ if name == Subcmd::Init.name() => Some(Subcmd::Init),
            _ if name == Subcmd::New.name() => Some(Subcmd::New),
            _ => None,
        }
    }
}

// Re-export to cmd module
pub use self::init::CmdInit;
pub use self::new::CmdNew;
