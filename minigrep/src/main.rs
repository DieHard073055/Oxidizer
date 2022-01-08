use std::env;
use std::process;

use ::minigrep;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(
        |err| {
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error {}", e);
        process::exit(1);
    }
}
