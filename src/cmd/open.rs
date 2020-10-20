use clap::Clap;

#[derive(Clap)]
pub struct OpenCmd {
    /// The name of the project to open.
    pub name: String,
}
