use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        // If the IGNORE_CASE environment variable isn’t set to anything, is_ok
        // will return false
        // - export IGNORE_CASE=1 // set
        // - echo $IGNORE_CASE // check, make sure to preface with the '$'
        // - unset IGNORE_CASE // unset
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("[lines] -> [{}]", line);
    }
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
    // Currently we will FAIL because we are ALWAYS returning an EMPTY vector
    // To fix that and implement search, our program needs to follow these steps:
    // -Iterate through each line of the contents.
    // -Check whether the line contains our query string.
    // -If it does, add it to the list of values we’re returning.
    // -If it doesn’t, do nothing.
    // -Return the list of results that match.
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    if results.is_empty() {
        println!("[NO RESULTS FOUND] for [{query}]");
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // transform query to lowercase and neutralize case sensitivity
    // Note that query is now a String rather than a string slice, because
    // calling to_lowercase creates new data rather than referencing existing
    // data
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    if results.is_empty() {
        println!("No results found for [{}]", query);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        // ensure that we don’t accidentally break the case-sensitive search
        // functionality that we’ve already implemented, but adding :
        // - "Duct tape." to the contents
        // using a capital D that shouldn’t match the query "duct" when we’re
        // searching in a case-sensitive manner
        // So that we can test both cases, we’ll add a new test function that
        // calls search with a case-sensitive query and assert that the search
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
