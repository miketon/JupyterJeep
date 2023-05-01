// returns an iterator from command line arguments
// iterators produce a series of values and can call the collect method
// to capture those values into a collection
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // export from main to sub function because
    // - subdivides scope to RELEASE closure only LOCAL variables
    // - makes it easier to test, enforces single responsibility
    let config = Config::new(&args);

    // the following output will be ...
    println!("Searching for {}", config.query); // Searching for needle
    println!("In file {}", config.file_path); // In file haystack.txt

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    dbg!(args);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        // &args[0] is the name of the program, we want the FIRST argument
        // There’s a tendency among many Rustaceans to avoid using clone to fix
        // ownership problems because of its runtime cost. In Chapter 13, you’ll
        // learn how to use more efficient methods in this type of situation.
        // But for now, it’s okay to copy a few strings to continue making progress
        // because you’ll make these copies only once and your file path and query
        // string are very small.
        // It’s better to have a working program that’s a bit inefficient than to
        // try to hyperoptimize code on your first pass. As you become more
        // experienced with Rust, it’ll be easier to start with the most efficient
        // solution, but for now, it’s perfectly acceptable to call clone.
        // @todo : learn more about clone, keep it simple for now
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
