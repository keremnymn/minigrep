use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    fn return_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => Ok(Self::return_config(&args)),
            _ => Err("There should be two arguments")
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}