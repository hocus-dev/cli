use clap::Clap;

#[derive(Clap)]
pub struct GetCmd {
    /// Git URL of the hocus project.
    pub url: String,
    /// How the project should be named locally.
    pub name: String,
}
