mod arrays;
mod conditions;
mod loops;
mod print;
mod strings;
mod tuples;
mod types;
mod utils;
mod vars;
mod vectors;
mod functions;
mod pointerz;
mod structz;
mod enums;
use colored::Colorize;
use eval::{eval};
// create a list of programs on the sandbox
static NAMES: [&str; 13] = [
    "print",
    "vars",
    "types",
    "strings",
    "tuples",
    "arrays",
    "vectors",
    "conditions",
    "loops",
    "functions",
    "pointers",
    "structs",
    "enums"
];
fn main() {
    main_menu();
}
/**
 * main menu
 */
fn main_menu() {
    println!(
        "{}",
        "welcome to rust sandbox. What would you like to do?"
            .bright_red()
            .bold()
            .on_truecolor(183, 65, 14)
            .to_string()
    );
    utils::print_tabbed("1. run sandbox programs".truecolor(183, 65, 14).to_string());
    utils::print_tabbed("2. eval rust expressions".truecolor(183, 65, 14).to_string());
    let input_text = utils::read_input();
    match input_text.parse::<i32>() {
        Ok(i) => main_switch(i),
        Err(..) => print_invalid_option(input_text)
    };
}
/**
 * using match, we execute either the sandbox programs or an eval expression
 */
fn main_switch(i:i32) {
    match i {
        // Match a single value
        1 => init_programs(),
        2 => init_eval(),
        // Handle the rest of cases
        _ => exit_sandbox(),
    };
}
/**
 * uses https://crates.io/crates/eval
 * to evaluate expressions in rust and return the resulting value
 */
fn init_eval() -> i32 {
    println!("{}", "Type your eval command:".bold().bright_green().italic().to_string());
    let input_text = utils::read_input();
    let eval_exp = eval(&input_text);
    match eval_exp {
        Ok(v) => println!("{}{}:{}", ">".bright_green().to_string(), input_text.dimmed().italic().to_string(),v),
        Err(e) => println!("error parsing expression: {e:?}"),
    }
    main_menu();
    return 0;
}
/**
 * creates a vector of program functions that return an i32.
 * then call for a read on the user input to execute the selected program on the list
 */
fn init_programs() -> i32 {
    let programs: Vec<fn() -> i32> = vec![
        print::run,
        vars::run,
        types::run,
        strings::run,
        tuples::run,
        arrays::run,
        vectors::run,
        conditions::run,
        loops::run,
        functions::run,
        pointerz::run,
        structz::run,
        enums::run
    ];
    return read_user_input(programs);
}
/**
 * reads the user input and executes a program
 */
fn read_user_input(programs: Vec<fn() -> i32>) -> i32 {
    // listing all programs and asking the user for input
    println!("Choose a program to run:");
    let mut input = 1;
    for x in NAMES {
        utils::print_tabbed(format!("{input}: {x}"));
        input += 1;
    }
    // reading option from user
    let input_text = utils::read_input();
    // validating the input and makes sure it's a positive integer
    match input_text.parse::<i32>() {
        Ok(i) => switch_input(programs, NAMES[(i as usize) - 1], i),
        Err(..) => print_invalid_option(input_text),
    };
    return 0;
}

/**
 * Handles user input and execute the appropriate program
 */
fn switch_input(programs: Vec<fn() -> i32>, program_name: &str, item: i32) {
    println!("{}", format!("You entered: {}", item).bold());
    // checking option passed is less than the length of the programs
    if item as usize > programs.len() {
        print_invalid_option(item.to_string());
        return;
    }
    println!("Executing {}...", program_name.red().italic());
    println!("{}", "{{".bold().green());
    programs[(item as usize) - 1]();
    println!("{}", "}}".bold().green());
    handle_program_end(program_name);
}
/**
 * what to do when the program ends
 */
fn handle_program_end(name: &str) {
    let pname = name.italic();
    println!();
    println!(
        "{}",
        format!("Program {} finished executing.", pname)
            .red()
            .bold()
    );
    println!("Press 1 to show menu. Press any key to exit");
    //getting user`s response input
    let input_text = utils::read_input();
    // validating the input and makes sure it's a positive integer
    match input_text.parse::<i32>() {
        Ok(i) => should_restart_program(i),
        Err(..) => println!("Exiting..."),
    };
}

/**
 * if input is 1, display list of programs to execute
 */
fn should_restart_program(i: i32) {
    if i == 1 {
        main_menu();
    } else {
        exit_sandbox();
    }
}
/**
 * standadizing the program exits
 */
fn exit_sandbox() -> i32 {
    println!("Exiting...");
    return 0;
}
/**
 * priting invalid options in red
 */
fn print_invalid_option(s:String) {
    println!("{}", format!("invalid option: {}", s).red())
}