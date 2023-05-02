use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // updating new to build, devs expect new to return an instance without FAIL
    // updated to return a Result as opposed to panicing
    // - Config when successful
    // - &str when error - a string slice initialized with a string literal.
    // String literals have a static lifetime, which means it is guaranteed to
    // be valid for the duration of the entire program...
    // @audit-ok : verify that this makes sense for an error message to have a
    // 'static lifetime
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("atleast 2 arguements needed -> query and file path");
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

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);
    // because this expression is about it's SIDE EFFECTS, we just returns ()
    Ok(())
}

// Notice that we need to define an explicit lifetime 'a in the signature of 
// search and use that lifetime with the contents argument and the return value
// - e tell Rust that the data returned by the search function will live as long 
// as the data passed into the search function in the contents argument
// @note : This is important! The data referenced by a slice needs to be valid 
// for the reference to be valid; if the compiler assumes we’re making string 
// slices of query rather than contents, it will do its safety checking 
// incorrectly
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    print!("Searching for {} {}", query, contents);
    // Currently we will FAIL because we are ALWAYS returning an EMPTY vector
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
