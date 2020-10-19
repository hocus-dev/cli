use super::subcmd::CmdInit;
use clap::{App, ArgMatches};

/// CLI argument handler.
pub struct Handler<'a> {
    /// The CLI matches.
    pub matches: ArgMatches<'a>,
}

impl<'a: 'b, 'b> Handler<'a> {
    pub fn build() -> App<'a, 'b> {
        App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .subcommand(CmdInit::build())
    }

    /// Parse CLI arguments.
    pub fn parse() -> Handler<'a> {
        // Obtain the program arguments
        #[allow(unused_mut)]
        let mut args: Vec<_> = ::std::env::args_os().collect();

        // Build the application CLI definition, get the matches
        Handler {
            matches: Handler::build().get_matches_from(args),
        }
    }
}
