#[path = "utils.rs"] mod utils;
pub fn run() -> i32 {
    //primitive strings = Immutable fixed-length string
    let user = "guinetik";
    //String = Growable, heap allocated data structure - Used when needed to modify    
    let mut greeting:String = String::from("Hello");
    // length
    utils::print_tabbed(
        format!("length: {}", greeting.len())
    );
    // pushing to Strings - push for character literal
    greeting.push(' ');
    // push strings
    greeting.push_str(user);
    utils::print_tabbed(
        format!("{}", greeting)
    );
    // capacity in bytes
    utils::print_tabbed(
        format!("Capacity: {}", greeting.capacity())
    );
    // is empty
    utils::print_tabbed(
        format!("is Empty: {}", greeting.is_empty())
    );
    // contains substring
    utils::print_tabbed(
        format!("Greeting contains {}: {}", user, greeting.contains(user))
    );
    //string replace
    utils::print_tabbed(
        format!("Replace: {}", greeting.replace("Hello", "Hi"))
    );
    //loop strings
    for word in greeting.split_whitespace() {
        utils::print_tabbed(
            format!("{}", word)
        );
    }
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    utils::print_tabbed(
        format!("{}", s)
    );
    //assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    return 0;
}