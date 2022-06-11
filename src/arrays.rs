#[path = "utils.rs"]
mod utils;
// Arrays - Fixed length where elements are the same data types
pub fn run() -> i32 {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    utils::print_tabbed(format!("{:?}", numbers));
    //get single value
    utils::print_tabbed(format!("First value: {}", numbers[0]));
    //mutable
    let mut numberz: [i32; 5] = [1, 2, 3, 4, 5];
    numberz[2] = 20;
    utils::print_tabbed(format!("{:?}", numberz));
    // array length
    utils::print_tabbed(format!("Array length: {}", numbers.len()));
    // Arrays are stack-allocated
    // & is used here so size_of_val borrows a reference to numbers
    utils::print_tabbed(format!(
        "Array is {} bytes",
        std::mem::size_of_val(&numbers)
    ));
    // slice array
    let slice: &[i32] = &numbers[0..2];
    utils::print_tabbed(format!("Slice: {:?}", slice));
    return 0;
}
