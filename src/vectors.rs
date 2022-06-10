#[path = "utils.rs"] mod utils;
pub fn run() -> i32 {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    utils::print_tabbed(format!("{:?}", numbers));
    //get single value
    utils::print_tabbed(format!("First value: {}", numbers[0]));
    // Vector length
    utils::print_tabbed(format!("Vector length: {}", numbers.len()));
    // pushing
    numbers.push(6);
    utils::print_tabbed(format!("{:?}", numbers));
    // pop last value
    numbers.pop();
    utils::print_tabbed(format!("{:?}", numbers));
    // loop through values
    for x in numbers.iter() {
        utils::print_tabbed(format!("Item: {}", x));
    }
    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    utils::print_tabbed(format!("Mutated Numbers: {:?}", numbers));
    return 0;
}