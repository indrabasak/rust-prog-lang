# Hello, World! Example


## Sample Code
The `main.rs` source file contains the following code,

```
fn main() {
    println!("Hello, world!");
}
```

The `main` is a special function which runs first in any Rust executable. It's
similar to C's and Java's main. In our example, it doesn't take parameters but
Rust `main` can also take parameters.

The `println!` is a Rust **macro** which prints out `Hello, world!`. If `println`
was a **function**, it should be `println` (without !).

## Compile
To compile the Rust program, execute the following command from a terminal,

```
rustc main.rs
```

It should create an executable named `main` in your directory (for Mac OS). 

## Run
The run the executable, run the `main` from the terminal,

```
$ ./main
```

It should print out the following
```
Hello, world!
```

