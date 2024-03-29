use std::{env, process};

use _rust_minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = _rust_minigrep::run(config) {
        eprintln!("Appication error: {e}");
        process::exit(1);
    }
}
