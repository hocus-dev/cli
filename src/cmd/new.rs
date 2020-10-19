use clap::Clap;

#[derive(Clap)]
pub struct NewCmd {
    /// How the project should be named.
    pub name: String,
}
