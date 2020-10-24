#[macro_use]
extern crate clap;
extern crate serde;

mod action;
mod cmd;
mod core;

use crate::action::invoke_action;
use crate::cmd::Root;
use crate::core::init::init;
use clap::Clap;

fn main() {
    let root = Root::parse();

    if let Err(err) = init() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }

    if let Err(err) = invoke_action(root) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
