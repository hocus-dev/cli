use clap::Clap;

pub mod close;
pub mod edit;
pub mod init;
pub mod new;
pub mod open;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Root {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    /// Initializes hocus on this machine
    Init(init::InitCmd),
    /// Initializes a new project
    New(new::NewCmd),
    /// Opens a project for editing
    Edit(edit::EditCmd),
    /// Opens a project
    Open(open::OpenCmd),
    /// Closes a project
    Close(close::CloseCmd),
}
