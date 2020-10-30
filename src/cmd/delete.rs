use clap::Clap;

#[derive(Clap)]
pub struct DeleteCmd {
    /// The name of the project to delete.
    pub name: String,
}
