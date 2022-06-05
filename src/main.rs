mod print;
mod vars;
mod types;
//
fn main() {
    println!("basic print commands");
    print::run();
    //
    println!("variables");
    vars::run();
    //
    println!("types in Rust");
    types::run();
}
