use std::io;

pub fn parse_input() -> String {
    let mut user_input = String::new();

    println!("Provide a word:");
    let _ = io::stdin().read_line(&mut user_input); // get string from the user input

    return user_input;
}