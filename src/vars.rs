//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language
pub fn run(){
    let name = "guinetik";
    //mutability with the mut keyword
    let mut age = 33;
    println!("user name is {}", name);
    println!("{} is {} years old", name, age);
    age = 34;
    println!("BIRTHDAY DETECTED! {} is {} years old", name, age);
    //constants
    const ID: i32 = 123456789;
    println!("ID: {}", ID);
    // multiple variable assignment
    let (total_mean, total_acc) = (100,1001);
    println!("total_mean: {} - total_acc: {}", total_mean, total_acc);
}