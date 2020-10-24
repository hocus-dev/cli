use clap::Clap;

#[derive(Clap)]
pub struct GetCmd {
    /// Git URL of the hocus environment.
    pub url: String,
    /// How the project should be named locally.
    pub name: String,
}
