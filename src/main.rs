#[macro_use]
extern crate clap;

mod action;
mod cmd;
mod core;

use crate::action::invoke_action;
use crate::cmd::Root;
use clap::Clap;

fn main() {
    let root = Root::parse();
    if let Err(err) = invoke_action(root) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
