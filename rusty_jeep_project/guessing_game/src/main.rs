use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // by default rust variables are immutable
    // mut keyword makes it mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed: {guess}");
}