use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //Handles argument input and collect to put it in a String vector
    dbg!(&args); //Remove when done

    // Declaring a config variable that calls parse_config -> Config
    let config = parse_config(&args);

    hash_print(50);
    println!("Searching for {}", config.query); //config.<attrib> is easier to understand relationship
    println!("In file {}", config.file_path); //config.<attrib> is easier to understand relationship
    hash_print(50);

    // Takes the string file_path and reads the text in it
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

// Parsing the cli arguments
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

// UTILS
fn hash_print(num: u8) {
    let str = "#".repeat(num as usize);
    println!("{}", str)
}
