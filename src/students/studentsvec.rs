use std::slice::Iter;

use super::{student::Student, Students};

pub struct StudentsVec {
    students: Vec<Student>,
}

impl StudentsVec {
    pub fn new() -> Self {
        return StudentsVec {
            students: Vec::new(),
        };
    }
}

impl Students for StudentsVec {
    fn all(&self) -> Iter<'_, Student> {
        return self.students.iter();
    }

    fn add(&mut self, s: Student) {
        self.students.push(s);
    }

    fn remove(&mut self, id: usize) {
        self.students.retain(|el| el.id() != id);
    }
}
