#[macro_use]
extern crate clap;

mod action;
mod cmd;
mod core;

use crate::action::invoke_action;
use crate::cmd::Handler;

fn main() {
    let cmd_handler = Handler::parse();
    if let Err(err) = invoke_action(cmd_handler) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
