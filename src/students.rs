mod idgenerator;

pub mod student;
pub mod studentsfile;
pub mod studentsvec;

use std::slice::Iter;
use student::Student;

use crate::utils::ask_stdin_input;

const COLUMN_WIDTH: usize = 20;
const TABLE_WIDTH: usize = 38 + COLUMN_WIDTH * 4;

pub trait Students {
    fn all(&self) -> Iter<'_, Student>;

    fn add(&mut self, s: Student);
    fn remove(&mut self, id: usize);
}

pub fn as_table_to_stdout(students: Iter<'_, Student>) {
    println!("{}", "-".repeat(TABLE_WIDTH));

    for s in students {
        println!("| Id: {:>width$} | Name: {:>width$} | Surname: {:>width$} | Date: {:>width$} |", 
            s.id(),
            s.name(),
            s.surname(),
            s.birth_date()
                .format(student::DATE_FORMAT)
                .to_string(),
            width = COLUMN_WIDTH);
    }

    println!("{}", "-".repeat(TABLE_WIDTH));
}

pub fn ask_and_delete_by_id(students: &mut dyn Students) {
    loop {
        let asked_string = ask_stdin_input("Id: ");

        let id = match asked_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        return students.remove(id);
    }
}
