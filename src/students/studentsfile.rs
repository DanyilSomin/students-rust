use super::{
    student::{self, Student},
    Students,
};

use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufWriter, Write},
};

pub struct StudentsFile {
    file_path: String,
    students: Vec<Student>,
    modified: bool,
}

impl StudentsFile {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let students = match read_students_from_file(file_path) {
            Ok(students) => students,
            Err(err) => return Err(err),
        };

        return Ok(StudentsFile {
            file_path: String::from(file_path),
            students,
            modified: false,
        });
    }
}

impl Drop for StudentsFile {
    fn drop(&mut self) {
        if !self.modified {
            return;
        }

        match write_students_to_file(self) {
            Some(err) => println!(
                "Failet to write students to file: {}",
                err.to_string()
            ),
            None => println!("Saved students to: {}.", self.file_path),
        }
    }
}

fn read_students_from_file(file_path: &str) -> io::Result<Vec<Student>> {
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file_path)
    {
        Ok(file) => file,
        Err(err) => {
            return Err(err);
        }
    };

    let mut students: Vec<Student> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let str = match line {
            Ok(str) => str,
            Err(err) => return Err(err),
        };

        let student = match student::Student::from_str(&str) {
            Some(student) => student,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to parse student.",
                ))
            }
        };

        students.push(student);
    }

    return Ok(students);
}

fn write_students_to_file(students: &StudentsFile) -> Option<io::Error> {
    let file = match OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .truncate(true)
        .open(&students.file_path)
    {
        Ok(file) => file,
        Err(err) => return Some(err),
    };

    let mut writer = BufWriter::new(file);

    for student in &students.students {
        match write!(&mut writer, "{}\n", student.to_string()) {
            Ok(_) => {}
            Err(err) => return Some(err),
        }
    }

    return None;
}

impl Students for StudentsFile {
    fn all(&self) -> std::slice::Iter<'_, Student> {
        return self.students.iter();
    }

    fn add(&mut self, s: Student) {
        self.students.push(s);
        self.modified = true;
    }

    fn remove(&mut self, id: usize) {
        self.students.retain(|el| el.id() != id);
        self.modified = true;
    }
}
