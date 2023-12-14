use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //Handles argument input and collect to put it in a String vector
    dbg!(&args); //Remove when done

    let query = &args[1];
    let file_path = &args[2];

    hash_print(50);
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    hash_print(50);

    // Takes the string file_path and reads the text in it
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

fn hash_print(num: u8) {
    let str = "#".repeat(num as usize);
    println!("{}", str)
}
