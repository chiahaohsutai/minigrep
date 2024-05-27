use minigrep::Config;
use std::{env, process};

// To run (case-insensitive):
// IGNORE_CASE=1 cargo run -- to sample.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_| {
        process::exit(1);
    });

    if let Err(_) = minigrep::run(config) {
        process::exit(1);
    }
}
