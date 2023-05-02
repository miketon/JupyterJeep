use std::env;
use std::process;
// @note : Ah interesting that the lib is pathed as the name of the project
// allows us to extract the logic into src/lib.rs
// - @note : leaves the argument collecting and error handling in src/main.rs
// - also easier to unit test in lib.rs
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {err}");
        process::exit(1);
    });

    // Because run returns () in the success case, we only care about detecting
    // an error, so we don’t need unwrap_or_else to return the unwrapped value,
    // which would only be () ... run(config) only about the SIDE EFFECTS
    // not the return value
    // @note : prefix run with our crate name -> minigrep
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
