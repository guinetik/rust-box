#[path = "utils.rs"] mod utils;
pub fn run() -> i32 {
    //tuples group together values of different types
    //max 12 elements
    let person: (&str, &str, i8) = ("guinetik", "brazil", 33);
    utils::print_tabbed(
        format!("{} from {} is {} years old", person.0, person.1, person.2)
    );
    return 0;
}