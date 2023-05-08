fn main() {
    let s1 = String::from("I am going to borrow this string");
    let string_len = calculate_length(&s1);
    eprintln!("Borrowed string: {} {}", string_len, s1);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here s goes out of scope, but because it does not have OWNERSHIP of what
  // it refers to, the actual string value of s1 is NOT DROPPED

/*
const HELLO_WORLD: &str = "Hi Hello, world! Goodby Amy!";
fn main() {
    println!("{}", HELLO_WORLD);
    let first = first_word(&HELLO_WORLD.to_string());
    println!("Waiting for new opp : {}", first);

    let x = 5;
    let x = x + 1; // Shadowing the previous x
    let x = x * 2; // Shadowing the previous x again

    println!("The value of x is: {}", x); // Output: The value of x is: 12

    // x = 42;          // Error: x is immutable, cannot assign twice to
                        //immutable variable
    let mut x = x; // Shadow to mutable
    x = 42;

    println!("The value of mutable x is: {}", x); // Output: x = 42
}

fn first_word(s : &String) -> usize{
    // because we need to got through element by element to check for a space,
    // we'll convert the string to an array of bytes
    let bytes = s.as_bytes();

    // iter through each element and return an index and an element as a tuple
    for (i, &item) in bytes.iter().enumerate(){
        // checking byte literal that represents a space
        if item == b' '{
            // a space indicates a complete word, return all chars up to this point
            return i;
        }
    }

    // no space found, will assume the entire string is a single word
    s.len()
}
*/
