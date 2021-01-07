#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate clap;

#[macro_use]
mod macros;

#[macro_use]
mod log;
mod cli;

pub mod config;
pub mod drawing;
pub mod library;

mod completion;
mod component;
mod error;
mod generators;
mod index;
mod outlines;
mod packages;
mod pattern;
mod pinout;
mod symbol;
mod symbols;

pub use error::Result;

pub async fn run_cli() -> Result<()> {
    cli::run().await
}
