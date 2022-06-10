#[path = "utils.rs"] mod utils;
use colored::Colorize;
//
pub fn run() -> i32 {
    //Printing to console
    utils::print_tabbed(
        "Hello, world"
        .bright_red()
        .bold()
        .on_truecolor(183, 65, 14)
        .to_string()
    );
    // Expression formating
    utils::print_tabbed(format!("Number: {}", 1));
    // Multiple arguments
    utils::print_tabbed(format!("{} is {}!", "Rust", "cool").green().italic().to_string());
    //Positional arguments
    utils::print_tabbed(format!("{0} is {1}. Logged in as {0}", "guinetik", "admin").blue().to_string());
    //Named arguments
    utils::print_tabbed(format!("{name} is {role}. Logged in as {name}", name="guinetik", role="admin").on_white().red().to_string());
    //Argument traits
    utils::print_tabbed(format!("Binary: {:b} - Hex: {:x} - Octal: {:o}", 10, 10, 10).bright_green().bold().to_string());
    //Debug trait
    utils::print_tabbed(format!("{:?}", (10, true, "guinetik")).dimmed().to_string());
    //Math Expressions
    utils::print_tabbed(format!("10 + 10 = {}", 10 + 10).italic().to_string());
    return 0;
}