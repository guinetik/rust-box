#[path = "utils.rs"] mod utils;
//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language
pub fn run() -> i32 {
    let name = "guinetik";
    //mutability with the mut keyword
    let mut age = 33;
    utils::print_tabbed(format!("user name is {}", name));
    utils::print_tabbed(format!("{} is {} years old", name, age));
    age = 34;
    utils::print_tabbed(format!("BIRTHDAY DETECTED! {} is {} years old", name, age));
    //constants
    const ID: i32 = 123456789;
    utils::print_tabbed(format!("ID: {}", ID));
    // multiple variable assignment
    let (total_mean, total_acc) = (100, 1001);
    utils::print_tabbed(format!("total_mean: {} - total_acc: {}", total_mean, total_acc));
    return 0;
}
