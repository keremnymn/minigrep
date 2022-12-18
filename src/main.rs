use std::{process, env};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing the arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}\nIn file {}...\n", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}