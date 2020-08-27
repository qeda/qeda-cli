#[macro_use]
mod log;

fn main() {
    debug!("running QEDA in debug mode");
    if let Err(e) = qeda::run_cli() {
        err!("{}", e);
        e.chain().skip(1).for_each(|cause| errln!("{}", cause));
        std::process::exit(1);
    }
}
