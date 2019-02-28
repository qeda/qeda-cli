#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
extern crate error_chain;
extern crate termcolor;

#[macro_use]
mod log;
mod cli;
mod errors;

use errors::*;

pub fn run_cli() -> Result<()> {
    cli::run()
}
