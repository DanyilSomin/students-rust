use chrono::*;
use std::fmt;

use crate::utils::{ask_stdin_input, is_valid_name};

use super::idgenerator;

pub const DATE_FORMAT: &str = "%d.%m.%Y";

pub struct Student {
    name: String,
    surname: String,
    birth_date: naive::NaiveDate,
    id: usize,
}

impl Student {
    pub fn from_stdin() -> Self {
        return Student {
            name: ask_user_valid_name("Name"),
            surname: ask_user_valid_name("Surname"),
            birth_date: ask_user_birth_day(),
            id: idgenerator::generate_id(),
        };
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn surname(&self) -> &String {
        return &self.surname;
    }

    pub fn birth_date(&self) -> &NaiveDate {
        return &self.birth_date;
    }

    pub fn id(&self) -> usize {
        return self.id;
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if !s.ends_with(";") {
            return None;
        }

        let splitted = &s[0..s.len() - 1].split(",").collect::<Vec<&str>>();

        if splitted.len() != 3 {
            return None;
        }

        let birth_date =
            match NaiveDate::parse_from_str(splitted[2], &DATE_FORMAT) {
                Ok(date) => date,
                Err(_) => return None,
            };

        return Some(Student {
            name: String::from(splitted[0]),
            surname: String::from(splitted[1]),
            birth_date,
            id: idgenerator::generate_id(),
        });
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{},{},{};",
            self.name,
            self.surname,
            self.birth_date.format(&DATE_FORMAT).to_string()
        );
    }
}

fn ask_user_birth_day() -> NaiveDate {
    loop {
        let birth_date_str =
            ask_stdin_input(&format!("Birth date ({})", &DATE_FORMAT));

        let date = match NaiveDate::parse_from_str(
            birth_date_str.as_str(),
            &DATE_FORMAT,
        ) {
            Ok(date) => date,
            Err(_) => {
                continue;
            }
        };

        if date > Utc::now().naive_utc().date() {
            continue;
        }

        return date;
    }
}

fn ask_user_valid_name(str: &str) -> String {
    loop {
        let name = ask_stdin_input(str);

        if is_valid_name(&name) {
            return name;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_usage() {
        let name = String::from("Name");
        let surname = String::from("Surname");
        let birth_date = chrono::naive::NaiveDate::from_ymd(2001, 12, 03);

        let student_str = format!(
            "{},{},{};",
            &name,
            &surname,
            birth_date.format(&DATE_FORMAT).to_string()
        );

        let s = match Student::from_str(&student_str) {
            Some(student) => student,
            None => panic!("Failed to parse student from string."),
        };

        assert_eq!(s.name(), &name);
        assert_eq!(s.surname(), &surname);
        assert_eq!(s.birth_date(), &birth_date);

        assert_eq!(student_str, s.to_string());
    }

    #[test]
    fn from_invalid_string() {
        let without_semicolon_str = "Name,Surname,12.03.2001";
        let without_date_str = "Name,Surname;";
        let without_surname_str = "Name,12.03.2001;";
        let wrong_separator_str = "Name Surname 12.03.2001;";

        match Student::from_str(without_semicolon_str) {
            Some(_) => panic!("Should be invalid."),
            None => {}
        };

        match Student::from_str(without_date_str) {
            Some(_) => panic!("Should be invalid."),
            None => {}
        };

        match Student::from_str(without_surname_str) {
            Some(_) => panic!("Should be invalid."),
            None => {}
        };

        match Student::from_str(wrong_separator_str) {
            Some(_) => panic!("Should be invalid."),
            None => {}
        };
    }

    #[test]
    fn valid_string() {
        let valid_str = "Name,Surname,12.03.2001;";

        match Student::from_str(valid_str) {
            Some(_) => {}
            None => panic!("Should be valid."),
        };
    }
}
