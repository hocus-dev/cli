use clap::Clap;

#[derive(Clap)]
pub struct CloseCmd {
    /// The name of the project to close.
    pub name: String,
}
