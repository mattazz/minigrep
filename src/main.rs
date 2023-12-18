use std::env;
use std::error::Error;
use std::fs;
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

    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

// Config Struct impl fn new -> Creates a new config struct
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// UTILS
fn hash_print(num: u8) {
    let str = "#".repeat(num as usize);
    println!("{}", str)
}
