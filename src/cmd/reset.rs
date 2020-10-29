use clap::Clap;

#[derive(Clap)]
pub struct ResetCmd {
    #[clap(subcommand)]
    pub subcmd: ResetSubCommand,
}

#[derive(Clap)]
pub enum ResetSubCommand {
    /// Deletes all volumes associated with a project
    Volumes(ResetVolumesCmd),
}

#[derive(Clap)]
pub struct ResetVolumesCmd {
    /// A project to reset
    pub name: String,
}
