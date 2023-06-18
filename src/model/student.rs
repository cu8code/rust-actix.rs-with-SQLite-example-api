use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Student {
    pub name: String,
    pub email: String,
    pub enrollment_no: u32,
}

impl Student {
    pub fn new(name: String, email: String, enrollment_no: u32) -> Self {
        Self {
            name,
            email,
            enrollment_no,
        }
    }
}

impl Student {
    pub fn init(db: &Connection) -> Result<usize, rusqlite::Error> {
        db.execute(
            "create table if not exists students (
                 enrollment_no integer primary key,
                 name text not null,
                 email text not null unique
             )",
            (),
        )
    }

    pub fn create(db: &Connection, student: Student) -> Result<usize, rusqlite::Error> {
        let command = format!(
            "insert into students (enrollment_no,name,email) values ({},\"{}\",\"{}\")",
            student.enrollment_no, student.name, student.email
        );
        db.execute(command.as_str(), ())
    }

    pub fn read(db: &Connection, enrollment_no: u32) -> Result<Student, rusqlite::Error> {
        let command = format!(
            "select * from students where enrollment_no = {}",
            enrollment_no
        );
        let mut smt = db.prepare(&command)?;
        let student_iter = smt.query_map([], |row| {
            Ok(Student::new(
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(0).unwrap(),
            ))
        });
        let data = student_iter
            .unwrap()
            .map(|x| x.unwrap())
            .collect::<Vec<Student>>();
        match data.len() {
            0 => Err(rusqlite::Error::QueryReturnedNoRows),
            _ => Ok(data[0].clone()),
        }
    }

    pub fn delete(db: &Connection, enrollment_no: u32) -> Result<usize, rusqlite::Error> {
        let command = format!(
            "delete from students where enrollment_no = {}",
            enrollment_no
        );
        db.execute(command.as_str(), ())
    }

    pub fn update(db: &Connection, enrollment_no: u32, email: String, name: String) -> Result<usize, rusqlite::Error> {
        let command = format!(
            "update students set name = \"{}\", email = \"{}\" where enrollment_no = {}",
            name, email, enrollment_no
        );
        db.execute(command.as_str(), ())
    }
}
