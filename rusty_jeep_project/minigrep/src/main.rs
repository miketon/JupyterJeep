// returns an iterator from command line arguments
// iterators produce a series of values and can call the collect method
// to capture those values into a collection
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // with command line input : cargo run -- needle haystack.txt
    // save argument values into  variables
    let query = &args[1]; // needle
    let file_path = &args[2]; // haystack.txt

    // the following output will be ...
    println!("Searching for {query}"); // Searching for needle
    println!("In file {file_path}"); // In file haystack.txt

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    dbg!(args);
}
