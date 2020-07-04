#![recursion_limit = "1024"]

#[macro_use]
mod log;
mod cli;

#[macro_use]
mod macros;

pub mod library;
pub mod config;
pub mod drawing;

mod errors;
mod component;
mod symbols;
mod patterns;
mod geometry;
mod generators;
mod svg;
mod text;

pub use errors::Result;

pub fn run_cli() -> Result<()> {
    cli::run()
}
