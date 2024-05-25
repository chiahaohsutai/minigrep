use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!(
        "Searching for \"{}\" in file: {}.",
        config.query, config.file_path
    );

    let contents =
        fs::read_to_string(config.file_path).expect("Failed to read the file at path: {filepath}");

    println!("With text: {contents}");
}
