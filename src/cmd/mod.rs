use clap::Clap;

pub mod close;
pub mod edit;
pub mod get;
pub mod init;
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
    /// Opens a project for editing in VSCode
    Edit(edit::EditCmd),
    /// Brings up the Docker environment associated with a project and opens VSCode
    Open(open::OpenCmd),
    /// Brings down the Docker environment associated with a project
    Close(close::CloseCmd),
    /// Downloads a project
    Get(get::GetCmd),
}
