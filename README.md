# The Rust Programming Language

![](./img/rust.jpeg?style=centerme)

This project contains examples from the book titled ` The Rust Programming Language`
by`Steve Klabnik` and `Carol Nichols`. 

## Installation on Mac OS
Open a terminal and enter the following command:

```
$ curl http://sh.rustup.rs -sSf | sh
```

The installation automatically adds Rust to the simple path

### To Check Version
The Rust versionn can be checked by typing the following commoand in a terminal,

```
$ rustc --version
```

The version result should look similar to the one shown below,

```
rustc 1.24.1 (d3ae9a9e0 2018-02-27)
```

### Updating Rust
To update Rust to the latest version, execute the following command,

```
$ rustup update
```

The update process should spit out information similar to the ones shown below, 
```
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2018-07-20, rust version 1.27.2 (58cc626de 2018-07-18)
info: downloading component 'rustc'
 56.5 MiB /  56.5 MiB (100 %)  27.8 MiB/s ETA:   0 s                
info: downloading component 'rust-std'
 44.8 MiB /  44.8 MiB (100 %)  28.2 MiB/s ETA:   0 s                
info: downloading component 'cargo'
info: downloading component 'rust-docs'
info: removing component 'rustc'
info: removing component 'rust-std'
info: removing component 'cargo'
info: removing component 'rust-docs'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: checking for self-updates
info: downloading self-update

  stable-x86_64-apple-darwin updated - rustc 1.27.2 (58cc626de 2018-07-18)
```

The Rust version can be checked again to validate the update,

```
$ rustc --version
rustc 1.27.2 (58cc626de 2018-07-18)
```

### Check Cargo Installation
Cargo is Rust's build system and package manager. It builds code and handles
dependency management. It's similar to Maven in Java world. 

To check if cargo is installed in your, execute the following command,

```
$ cargo --version
```
It should spit out a message similar to the message shown below,
```
cargo 1.27.0 (1e95190e5 2018-05-27)
```