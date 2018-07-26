# Hello, Cargo Example

## Create a New Project with Cargo
To create a new cargio project titled `hello_cargo`, execute the following 
commad from aterminal,

```
 cargo new hello_cargo --bin
```

The `--bin` argument informs cargo to build a new binary exeuctable project
instead of a library. If the project is successfully created, it should p
rint the following message,
```
     Created binary (application) `hello_cargo` project
```

This should create a `hello_cargo` folder with two files,
   - Cargo.toml
   - main.rs
   - .gitignore
   
### Cargo.toml
TOML stands for Tom's Obvious Minimal Language. It `Cargo.toml` cantains all the
information related to projects's build and dependency management. Here's the
content of `Cargo.toml`.

```
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["indra.basak <indra.basak1@gmail.com>"]

[dependencies]
```

The `package` section contains information about project `name`, `version`, and the
`author`. The `dependencies` section usually contains a project dependencies.
Since the `hello_cargo` is a trivial project, it doesn't contain any dependencies.
In Rust, a libray (apckage of code) is referred as a `crate`.

### Build and Running a Cargo Project
To build, execute the following command from the `hello_cargo` directory,
```
$ cargo build
```

Wile building, it should print similar messages a shwon below,

```
   Compiling hello_cargo v0.1.0 (file:///.../rust-prog-lang/chap1/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.95s
```

Once the builkd is completed successfully, it should create an excutable
file `target/debug/hello_cargo`. To run the executable, run the following
from a  terminal.    

```
$ ./target/debu`g/hello_cargo
```

I should print out,
```
Hello, world!
```

The project can be buit and run using a single command,
```
$ cargo run
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello_cargo`
Hello, world!
```

Since the source code wasn't modified, it just runs the executable. if the file
was modified, the `cargo run` would have recompiled before executing.

The `cargo check` check the code syntax by compiling buut without producing an executable
```
$ cargo check
```

```
    Checking hello_cargo v0.1.0 (file:///.../rust-prog-lang/chap1/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
```    

To build an optimized executable for release, run the folwoing comamand,

```
 $ cargo build --release
 ```
 
It should produce an optimized executable under `target/release` directory.
 ```
   Compiling hello_cargo v0.1.0 (file:///.../rust-prog-lang/chap1/hello_cargo)
    Finished release [optimized] target(s) in 0.44s
```