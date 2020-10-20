use clap::Clap;

#[derive(Clap)]
pub struct EditCmd {
    /// The name of the project to edit.
    pub name: String,
}
