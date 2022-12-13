use std::{process, fs, env};

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn return_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
    fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => Ok(Self::return_config(&args)),
            _ => Err("There should be two arguments")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error while parsing the arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}\nIn file {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
                                                .expect("Couldn't read the file.");

    println!("With text:\n{contents}");
}