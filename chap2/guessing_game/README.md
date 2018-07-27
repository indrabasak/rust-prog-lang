

## Guessing Game - Part 1

```
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
```

### Running

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
5
You guessed: 5
```
## Adding a Crate

```
[dependencies]

rand = "0.3.14"
```

```
 cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading libc v0.2.42                                                       
   Compiling libc v0.2.42                                                       
   Compiling rand v0.4.2
   Compiling rand v0.3.22
   Compiling guessing_game v0.1.0 (file:///Users/indra.basak/Development/examples/rust-prog-lang/chap2/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 44.45
```

### Updating a Crate

```
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`

```