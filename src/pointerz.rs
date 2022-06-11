#[path = "utils.rs"]
mod utils;
// Reference Pointers - Point to a resource in memory
pub fn run() -> i32 {
    //primitive array;
    let arr1 = [1,2,3];
    let arr2 = arr1;
    utils::print_tabbed(format!("arr1 and arr2 are equal? {:?} - {:?}", arr1, arr2));
    /* 
        With non-primitives, if you assign another variable to a piece of data, 
        the first variable (left side) will no longer hold that value.
        You'll need to use a reference (&) to point to the resource
        https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html 
    */
    let vec1 = vec![1,2,3];
    let vec2 = &vec1; // if we dont use '&' here, vec1 becomes sort of NULL 'move occurs because `vec1` has type `Vec<i32>`, which does not implement the `Copy` trait'
    //the nice thing about Rust, is that it let's you know that in compile time, so you don't ship phantom bugs do production.
    utils::print_tabbed(format!("vectors {:?} - {:?}", vec1, vec2));
    return 0;
}