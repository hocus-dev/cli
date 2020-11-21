use clap::{AppSettings, Clap};

pub mod analytics;
pub mod close;
pub mod delete;
pub mod edit;
pub mod get;
pub mod open;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Root {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    /// Opens a project for editing in VSCode
    Edit(edit::EditCmd),
    /// Brings up the Docker environment associated with a project and opens VSCode
    Open(open::OpenCmd),
    /// Brings down the Docker environment associated with a project
    Close(close::CloseCmd),
    /// Downloads a project
    Get(get::GetCmd),
    /// Deletes a project
    Delete(delete::DeleteCmd),
    /// Send a request to the analytics service. Used internally by Hocus itself to open a new
    /// process that handles telemetry.
    #[clap(setting(AppSettings::Hidden))]
    Analytics(analytics::AnalyticsCmd),
}
