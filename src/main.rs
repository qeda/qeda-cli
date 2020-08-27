#[macro_use]
mod log;

fn main() {
    debug!("running QEDA in debug mode");
    if let Err(e) = qeda::run_cli() {
        err!("{}", e);
        for e in e.iter().skip(1) {
            errln!("{}", e);
        }
        std::process::exit(1);
    }
}
