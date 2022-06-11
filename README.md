# rust-box
Studies on the Rust language.

These are examples I wrote while following along the video Rust Crash Course by Traversy Media on (https://www.youtube.com/watch?v=zF34dRivLOw&t=5071s).  

From there I created the main structure to choose a program to run and also the option to eval expressions by using the eval crate (https://crates.io/crates/eval)

# Installing
Since I'm on windows, I used chocolatey pkg manager and installed 2 pkgs:

rust compiler and pkg manager:
`choco install rust`

rust up server:
`choco install rustup.install`

# Compiling
Invoke rust's compiler CLI with:

`rustc main.rs`

# Cargo = rust pkg manager
In cargo, packages = crates.

Create a new project on current folder:

`cargo init`

Create a new project on a new folder:

`cargo new project-name`

To run a project:

`cargo run`

To build a project:
`cargo build`

To build for release:

`cargo build --release`