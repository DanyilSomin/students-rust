use std::io::{self, Write};

pub fn ask_stdin_input(ask_what: &str) -> String {
    print!("{}: ", &ask_what);
    io::stdout().flush().unwrap();

    let mut asked_string = String::new();
    io::stdin()
        .read_line(&mut asked_string)
        .expect("Failed to read line");

    return String::from(asked_string.trim());
}
