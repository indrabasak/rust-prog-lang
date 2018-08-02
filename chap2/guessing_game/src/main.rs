extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("the secret number is: {}", secret_number);

    // allowing multiple guesses with looping
    loop {
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

        // convert the string to an unsigned int to
        // avoid mismatched types error
        // let guess: u32 = guess.trim().parse()
         //   .expect("Please type a  number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // message formatting
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //Ordering::Equal => println!("You win!")
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
