
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // in Rust, variables are immutable by default
    // 'mut' is used to make variable 'guess' mutable
    // String::new() creates a new instance of a string
    // :: associated function of String type
    // associated function is imnplemented on a type instead
    // of a particular instance of a String
    let mut guess = String::new();

    // '&'; indicates reference
    // 'mut &' mutable reference
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // message formatting
    println!("You guessed: {}", guess);
}
