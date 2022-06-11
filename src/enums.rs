use colored::Colorize;
#[path = "utils.rs"]
mod utils;
#[path = "structz.rs"]
mod structz;
use rand::seq::SliceRandom;
use std::panic;
#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

enum DificultyLevels {
    EASY,
    NORMAL,
    HARD,
    INSANE,
}
/**
 * Here we create a mapping function to give each dificulty a color...for the dramatic effect
 */
fn dificulty_color(level: &DificultyLevels) -> structz::Color {
    match level {
        DificultyLevels::EASY => structz::Color {
            red: 0,
            green: 255,
            blue: 50,
        },
        DificultyLevels::NORMAL => structz::Color {
            red: 0,
            green: 0,
            blue: 255,
        },
        DificultyLevels::HARD => structz::Color {
            red: 255,
            green: 50,
            blue: 50,
        },
        DificultyLevels::INSANE => structz::Color {
            red: 255,
            green: 0,
            blue: 0,
        }
    }
}

fn dificulty_label(level: &DificultyLevels) -> String {
    match level {
        DificultyLevels::EASY => "Easy".to_string(),
        DificultyLevels::NORMAL => "Normal".to_string(),
        DificultyLevels::HARD => "Hard".to_string(),
        DificultyLevels::INSANE => "InSaNe!".to_string()
    }
}

fn dificulty_size(level: &DificultyLevels) -> u8 {
    match level {
        DificultyLevels::EASY => 2,
        DificultyLevels::NORMAL => 3,
        DificultyLevels::HARD => 5,
        DificultyLevels::INSANE => 10
    }
}

fn move_avatar(m:Movement) {
    match m {
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run() -> i32 {
    setup_game();
    return 0;
}

fn setup_game() {
    utils::print_tabbed("let`s play a game...choose your torment:".black().on_red().bold().italic().to_string());
    let dificulties = [DificultyLevels::EASY, DificultyLevels::NORMAL, DificultyLevels::HARD, DificultyLevels::INSANE];
    for x in 0..dificulties.len() {
        let d = &dificulties[x];
        let c = dificulty_color(&d);
        let l = dificulty_label(&d);
        utils::print_tabbed(format!("{}. {}", x, l).on_truecolor(c.red, c.green, c.blue).bold().italic().to_string());
    }
    let input_text = utils::read_input();
    match input_text.parse::<u8>() {
        Ok(i) => play(i, dificulties),
        Err(..) => print_invalid_option(input_text),
    };
}

fn play(dificulty:u8, dificulties:[DificultyLevels;4]) {
    if dificulty as usize > dificulties.len() {
        print_invalid_option(dificulty.to_string());
        return;
    }
    // creating an array of directions
    let directions = [Movement::Up, Movement::Down, Movement::Left, Movement::Right];
    // constructing the level based on dificulty
    let mut level:Vec<&Movement> = vec![];
    let d = &dificulties[dificulty as usize];
    // each dificulty has a set number of tiles for the level
    let number_of_tiles = dificulty_size(d);
    // for each tile, pick a random direction to go. this would be the "correct" input to go forward.
    for x in 0..number_of_tiles {
        let random_direction = directions.choose(&mut rand::thread_rng()).unwrap();
        level.push(random_direction);
    }
    //
    //utils::print_tabbed(format!("{:?}", level));
    //
    utils::print_tabbed("You wake up on a dark room of a mansion.".black().on_purple().bold().italic().to_string());
    utils::print_tabbed("This room has exits on all directions...".black().on_purple().bold().italic().to_string());
    //
    let mut total_moves:u8 = 0;
    let mut alive = true;
    while alive && (total_moves as usize) < level.len() {
        (alive, total_moves) = game_loop(&directions, &level, total_moves);
    }
    if alive {
        utils::print_tabbed("You WIN! Trully a survivor!".on_blue().yellow().bold().italic().to_string());
    } else {
        utils::print_tabbed("You died!".black().on_red().italic().to_string());
    }
}

fn game_loop(directions:&[Movement;4], level:&Vec<&Movement>, mut total_moves:u8) -> (bool, u8) {
    let current_move = get_input_from_user(&directions);
    let d = &directions[current_move as usize];
    if level[total_moves as usize] == d {
        total_moves = total_moves+1;
        utils::print_tabbed("You survived to the next room!".yellow().to_string());
        (true, total_moves)
    } else {
        (false, total_moves)
    }
}

fn get_input_from_user(directions:&[Movement;4]) -> u8 {
    utils::print_tabbed("Where do you move?".green().to_string());
    for x in 0..directions.len() {
        let d = &directions[x];
        utils::print_tabbed(format!("{}. {:?}", x, d).bold().italic().to_string());
    }
    let user_choice = get_value_from_input();
    return user_choice;
}

fn print_invalid_option(s:String) {
    println!("{}", format!("invalid option: {}", s).red())
}

fn get_value_from_input() -> u8 {
    let result = panic::catch_unwind(|| {
        let c:u8 = utils::read_input().trim().parse().expect(&"Wanted a number".red().bold());
        c
    });
    if result.is_ok() {
        return result.unwrap();
    }
    return 0;
}