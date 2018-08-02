Chapter 3
============

## Variables and Mutability


## Not allowed to change a mutable varaible type

```
error[E0308]: mismatched types
  --> src/main.rs:36:16
   |
36 |     spaces_1 = spaces_1.len();
   |                ^^^^^^^^^^^^^^ expected &str, found usize
   |
   = note: expected type `&str`
              found type `usize`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: Could not compile `variables`.
```

## Unused Variable Warning

```rust
let guess : u32 = "42".parse().expect("Not a number!");
```

```
warning: unused variable: `guess`
  --> src/main.rs:38:9
   |
38 |     let guess : u32 = "42".parse().expect("Not a number!");
   |         ^^^^^ help: consider using `_guess` instead
   |
   = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/variables`
```

No unused variable error
```rust
let _guess : u32 = "42".parse().expect("Not a number!");
```

## Missing Annotation Type Error
```rust
let guess = "42".parse().expect("Not a number!");

```
error[E0282]: type annotations needed
  --> src/main.rs:40:9
   |
40 |     let guess = "42".parse().expect("Not a number!");
   |         ^^^^^
   |         |
   |         cannot infer type for `_`
   |         consider giving `guess` a type

error: aborting due to previous error
```