#![recursion_limit = "1024"]

#[macro_use]
mod log;

use std::process;
use qeda;

fn main() {
    debug!("running QEDA in debug mode");
    if let Err(e) = qeda::run_cli() {
        err!("{}", e);
        for e in e.iter().skip(1) {
            errln!("{}", e);
        }
        process::exit(1);
    }
}
