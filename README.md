# rust-box
Studies on the Rust language

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

