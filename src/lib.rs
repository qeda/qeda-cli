#[macro_use]
extern crate bitflags;

#[macro_use]
mod log;
mod cli;

#[macro_use]
mod macros;

pub mod config;
pub mod drawing;
pub mod library;

mod component;
mod error;
mod generators;
mod geometry;
mod patterns;
mod pin;
mod svg;
mod symbols;
mod text;

pub use error::Result;

pub fn run_cli() -> Result<()> {
    cli::run()
}
