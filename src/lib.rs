#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
extern crate error_chain;
extern crate termcolor;
extern crate reqwest;

#[macro_use]
mod log;
mod cli;

#[allow(deprecated)] // See https://github.com/rust-lang-nursery/error-chain/issues/254
mod errors;
mod library;
mod component;

pub use errors::*;

pub fn run_cli() -> Result<()> {
    cli::run()
}
