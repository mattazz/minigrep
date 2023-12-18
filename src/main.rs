use minigrep::{hash_print, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); //Handles argument input and collect to put it in a String vector

    // Declaring a config variable that calls parse_config -> Config
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    hash_print(50);
    println!("Searching for {}", config.query); //config.<attrib> is easier to understand relationship
    println!("In file {}", config.file_path); //config.<attrib> is easier to understand relationship
    hash_print(50);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
