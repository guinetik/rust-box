use std::io;
pub fn print_tabbed(line:String) {
    let result:String = "\t".to_owned() + &line;
    println!("{}", result);
}

pub fn read_input() -> String {
    let mut input_text = String::new();
    // input is handled by io pkg
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    // trimming the input
    return input_text.trim().to_string();
}