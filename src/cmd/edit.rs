use clap::Clap;

#[derive(Clap)]
pub struct EditCmd {
    /// The name of the project to open for editing.
    pub name: String,
}
