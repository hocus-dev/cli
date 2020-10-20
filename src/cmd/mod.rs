use clap::Clap;

pub mod edit;
pub mod init;
pub mod new;

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
}
