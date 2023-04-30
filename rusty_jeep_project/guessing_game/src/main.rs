use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Guess the secret number!");

    // - Call the rand::thread_rng function that gives us the particular random 
    // number generator we’re going to use: one that is local to the current 
    // thread of execution and is seeded by the operating system. 
    // - Then call the gen_range method on the random number generator. This 
    // method is defined by the Rng trait that we brought into scope with the 
    // cuse rand::Rng; statement
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");

    println!("Please input your guess.");

    // by default rust variables are immutable
    // mut keyword makes it mutable
    // String is a string type provided by the standard library that is a
    // growable, UTF-8 encoded bit of text : it MUST be MUTABLE else error
    let mut guess = String::new();

    // instance of std::io::Stdin is a type that represents a HANDLE to the
    // standard input for your terminal.
    io::stdin()
        // - The string "guess" argument needs to be mutable so the method
        // can CHANGE the string’s content as user types
        // - References are immutable by default. Hence, you need to write
        // "&mut guess" NOT "&guess" to make it MUTABLE
        .read_line(&mut guess)
        .expect("Failed to read line");

    // We create a variable named guess. But wait, doesn’t the program already 
    // have a variable named guess? It does, but helpfully Rust allows us to 
    // shadow the previous value of guess with a new one. Shadowing lets us 
    // reuse the guess variable name rather than forcing us to create two 
    // unique variables, such as guess_str and guess, for example
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    }
}
