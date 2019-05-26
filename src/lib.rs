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

#[macro_use]
mod macros;

pub mod library;
pub mod config;
pub mod drawing;

#[allow(deprecated)] // See https://github.com/rust-lang-nursery/error-chain/issues/254
mod errors;
mod component;
mod symbols;
mod patterns;
mod generators;

pub use errors::Result;

pub fn run_cli() -> Result<()> {
    cli::run()
}
