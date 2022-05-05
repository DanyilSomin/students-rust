mod students;
mod utils;

use students::{student::Student, studentsvec::StudentsVec, Students};

use crate::{students::studentsfile::StudentsFile, utils::ask_stdin_input};

fn main() {
    println!("HELLO STUDENT");

    let students = ask_user_and_create_students();

    main_loop(students);
}

fn ask_user_and_create_students() -> Box<dyn Students> {
    loop {
        let asked_string = ask_stdin_input("Choose DB (vec/file)");

        match asked_string.as_str() {
            "vec" => return Box::from(StudentsVec::new()),
            "file" => return ask_user_and_open_from_file(),
            _ => {}
        }
    }
}

fn ask_user_and_open_from_file() -> Box<dyn Students> {
    loop {
        let asked_string = ask_stdin_input("Path to file");

        match StudentsFile::new(&asked_string) {
            Ok(students) => return Box::from(students),
            Err(err) => {
                println!("Error: {}", err.to_string());
            }
        }
    }
}

fn main_loop(mut students: Box<dyn Students>) {
    loop {
        let asked_string =
            ask_stdin_input("Choose command (add/delete/print/quit)");

        match asked_string.as_str() {
            "print" => students::as_table_to_stdout(students.all()),
            "add" => students.add(Student::from_stdin()),
            "delete" => students::ask_and_delete_by_id(students.as_mut()),
            "quit" => break,

            _ => {}
        }
    }
}
