use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for \"{query}\" in file: {filepath}.");

    let contents =
        fs::read_to_string(filepath).expect("Failed to read the file at path: {filepath}");

    println!("With text: {contents}");
}
