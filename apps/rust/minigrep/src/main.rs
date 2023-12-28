use std::{env, process};

use _rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = _rust_minigrep::run(config) {
        println!("Appication error: {e}");
        process::exit(1);
    }
}
