use colored::Colorize;
#[path = "utils.rs"]
mod utils;
// a function is used to store blocks of codes to reuse
pub fn run() -> i32 {
    utils::print_tabbed("What's your name?".italic().to_string());
    let mut input_text = utils::read_input();
    greeting("Hello", &input_text);
    //
    utils::print_tabbed(format!(
        "{} {}",
        input_text.green().bold(),
        "let's add some numbers. Pick a number:"
    ));
    let s1 = utils::read_input();
    utils::print_tabbed("Pick another".to_string());
    let s2 = utils::read_input();
    //
    let n1 = s1.parse().unwrap();
    let n2 = s2.parse().unwrap();
    let sum = add(n1, n2);
    utils::print_tabbed(format!(
        "{}{}+{}={}",
        ">".bright_green().to_string(),
        s1.dimmed().italic().to_string(),
        s2.dimmed().italic().to_string(),
        sum
    ));
    // closures
    let add_closure = |a1:i32, a2:i32| a1 + a2;
    utils::print_tabbed(format!("{}: {}", "closure function |a1:i32, a2:i32| a1 + a2 = ", add_closure(n1, n2)));
    return 0;
}

fn greeting(greet: &str, name: &str) {
    utils::print_tabbed(format!(
        "{} {}. Nice to meet you",
        greet.blue().bold(),
        name.yellow().italic()
    ));
}

fn add(n1: i32, n2: i32) -> i32 {
    //ommiting the semicolon here signals to the compiler that this is the return value
    n1 + n2
}
