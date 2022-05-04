use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};

lazy_static! {
    static ref NAME_REGEX: Regex =
        Regex::new(r"^[a-zA-Z`'-]{2,25}$").unwrap();
}

pub fn ask_stdin_input(ask_what: &str) -> String {
    print!("{}: ", &ask_what);
    io::stdout().flush().unwrap();

    let mut asked_string = String::new();
    io::stdin()
        .read_line(&mut asked_string)
        .expect("Failed to read line");

    return String::from(asked_string.trim());
}

pub fn is_valid_name(str: &String) -> bool {
    return NAME_REGEX.is_match(str);
}
