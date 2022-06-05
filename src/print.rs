pub fn run(){
    //Printing to console
    println!("Hello, world");
    // Expression formating
    println!("Number: {}", 1);
    // Multiple arguments
    println!("{} is {}!", "Rust", "cool");
    //Positional arguments
    println!("{0} is {1}. Logged in as {0}", "guinetik", "admin");
    //Named arguments
    println!("{name} is {role}. Logged in as {name}", name="guinetik", role="admin");
    //Argument traits
    println!("Binary: {:b} - Hex: {:x} - Octal: {:o}", 10, 10, 10);
    //Debug trait
    println!("{:?}", (10, true, "guinetik"));
    //Math Expressions
    println!("10 + 10 = {}", 10 + 10)
}