#[path = "utils.rs"]
mod utils;
use colored::Colorize;
use std::panic;
use tabled::{Tabled, Table};
// Structs - Used to create custom data types

// Traditional Struct
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    // Constructor
    pub fn new (r:u8, g:u8, b:u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.red, self.green, self.blue)
    }

}

// Tuple Struct
struct Colorz(u8, u8, u8);

// Some sort of an entity
#[derive(Tabled)]
struct Person {
    first_name: String,
    last_name: String,
    dob: String,
    username: String,
}

impl Person {
    fn new(first: &str, last: &str, dob: &str, user: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            dob: dob.to_string(),
            username: user.to_string()
        }
    }

    fn create() -> Person {
        Person {
            first_name: "".to_string(),
            last_name: "".to_string(),
            dob: "".to_string(),
            username: "".to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Setters

    fn set_first_name(&mut self, value: &str) {
        self.first_name = value.to_string();
    }

    fn set_last_name(&mut self, value: &str) {
        self.last_name = value.to_string();
    }

    fn set_dob(&mut self, value: &str) {
        self.dob = value.to_string();
    }

    fn set_username(&mut self, value: &str) {
        self.username = value.to_string();
    }

    fn to_touple(&self) -> (String, String, String, String) {
        (String::from(&self.first_name), 
        String::from(&self.last_name),
        String::from(&self.dob),
        String::from(&self.username))
    }
}

pub fn run() -> i32 {
    utils::print_tabbed("Let's play with colors".color("blue").on_color("yellow").to_string());
    utils::print_tabbed("Pick a number for red (0 to 255)".color("white").on_color("red").to_string());
    // getting color from user input and making sure is 0 to 255
    let c1:u8 = get_color_from_input();
    utils::print_tabbed("Pick a number for green (0 to 255)".color("white").on_color("green").to_string());
    // getting blue
    let c2:u8 = get_color_from_input();
    utils::print_tabbed("Pick a number for blue (0 to 255)".color("white").on_color("blue").to_string());
    //
    let c3:u8 = get_color_from_input();
    let mut c = Color {
        red: c1,
        green: c2,
        blue: c3,
    };
    utils::print_tabbed("Your custom color is beautiful!".truecolor(c.red, c.green, c.blue).to_string());
    //
    let k = Colorz(c1, c2, c3);
    utils::print_tabbed(format!("Your color choices - red: {}, green: {}, blue: {}", k.0, k.1, k.2));
    //
    c = Color::new(c1, c2, c3);
    utils::print_tabbed(format!("This is the Hex for your color: {}", c.to_hex().truecolor(c1, c2, c3)));
    //
    let gui:Person = Person::new("Joao", "Guilherme", "1988-29-08", "guinetik");
    utils::print_tabbed(format!("My name is {}.", gui.full_name()));
    let friends = vec![gui];
    make_friends(friends, c);
    //
    return 0;
}

fn make_friends(mut friends:Vec<Person>, c:Color) {
    // creating new empty user and setting first name from user input
    let mut user = Person::create();
    utils::print_tabbed("What is your Name?".blue().to_string());
    user.set_first_name(&utils::read_input());
    // setting last name
    utils::print_tabbed("Last name:".blue().to_string());
    user.set_last_name(&utils::read_input());
    // setting dob
    utils::print_tabbed("DOB:".blue().to_string());
    user.set_dob(&utils::read_input());
    // setting username
    utils::print_tabbed("username:".blue().to_string());
    user.set_username(&utils::read_input());
    //
    utils::print_tabbed(format!("Hello, {:?}", Person::to_touple(&user)).bold().magenta().to_string());
    // add to list
    friends.push(user);
    let table = Table::new(&friends).to_string();
    println!();
    utils::print_tabbed("My Friends as rust Structs".bold().truecolor(c.red, c.green, c.blue).to_string());
    println!("{}", table.truecolor(c.red, c.green, c.blue));
    //
    utils::print_tabbed("Add more friends? [y]es [n]o".bold().to_string());
    let choice = utils::read_input();
    if "y" == choice || "yes" == choice {
        make_friends(friends, c);
    } else if "n" == choice || "no" == choice {
        utils::print_tabbed("Goodbye, friend!".bold().truecolor(c.red, c.green, c.blue).to_string());
    }
}

/**
 * Here we use catch_unwind to enforce type coersion on the input color (u8 is unsigned 8 btye so 0 to 255)
 * it's a little hacky, but does the job, I'm still learning the rust ropes
 */
fn get_color_from_input() -> u8 {
    let result = panic::catch_unwind(|| {
        let c:u8 = utils::read_input().trim().parse().expect(&"Wanted a number".red().bold());
        c
    });
    if result.is_ok() {
        return result.unwrap();
    }
    return 0;
}