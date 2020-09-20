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
mod packages;
mod pattern;
mod pinout;
mod symbol;
mod symbols;

pub use error::Result;

pub async fn run_cli() -> Result<()> {
    cli::run().await
}
