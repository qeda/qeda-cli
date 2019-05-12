#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
extern crate crypto_hash;
extern crate error_chain;
extern crate hex;
extern crate termcolor;
extern crate reqwest;

#[macro_use]
mod log;
mod cli;

#[allow(deprecated)] // See https://github.com/rust-lang-nursery/error-chain/issues/254
mod errors;
mod library;
mod component;
mod drawing;
mod schematic;
mod symbols;
mod utils;

pub use errors::*;

pub fn run_cli() -> Result<()> {
    cli::run()
}
