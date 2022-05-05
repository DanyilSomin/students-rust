mod idgenerator;

pub mod student;
pub mod studentsfile;
pub mod studentsvec;

use std::slice::Iter;
use student::Student;

use crate::utils::ask_stdin_input;

const NAME_COLUMN_WIDTH: usize = 25;
const ID_COLUMN_WIDTH: usize = 5;
const DATE_COLUMN_WIDTH: usize = 10;
const TABLE_WIDTH: usize =
    38 + NAME_COLUMN_WIDTH * 2 + ID_COLUMN_WIDTH + DATE_COLUMN_WIDTH;

pub trait Students {
    fn all(&self) -> Iter<'_, Student>;

    fn add(&mut self, s: Student);
    fn remove(&mut self, id: usize);
}

pub fn as_table_to_stdout(students: Iter<'_, Student>) {
    println!("{}", "-".repeat(TABLE_WIDTH));

    for s in students {
        println!("| Id: {:>id_width$} | Name: {:>name_width$} | Surname: {:>name_width$} | Date: {:>date_width$} |", 
            s.id(),
            s.name(),
            s.surname(),
            s.birth_date()
                .format(student::DATE_FORMAT)
                .to_string(),
            name_width = NAME_COLUMN_WIDTH,
            id_width = ID_COLUMN_WIDTH,
            date_width = DATE_COLUMN_WIDTH);
    }

    println!("{}", "-".repeat(TABLE_WIDTH));
}

pub fn ask_and_delete_by_id(students: &mut dyn Students) {
    loop {
        let asked_string = ask_stdin_input("Id");

        let id = match asked_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        return students.remove(id);
    }
}
