use crate::common::{app_error, Result};
use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct Employee {
    dept: String,
    name: String,
}

#[derive(Debug)]
pub enum DbCommand {
    AddEmployee(Employee),
}

impl Database {
    pub fn new() -> Database {
        Database { db: HashMap::new() }
    }

    pub fn make_employee(dept: String, name: String) -> Employee {
        Employee { dept, name }
    }

    pub fn get_departments(&self) -> Option<Vec<&String>> {
        if self.db.is_empty() {
            None
        } else {
            Some(self.db.keys().collect())
        }
    }

    pub fn get_departments_sorted(&self) -> Option<Vec<&String>> {
        self.get_departments().and_then(|mut depts| {
            depts.sort();
            Some(depts)
        })
    }

    pub fn get_employees_for_dept(&self, dept: &str) -> Option<Vec<&String>> {
        self.db
            .get(dept)
            .and_then(|employees| Some(employees.iter().collect()))
    }

    pub fn get_employees_for_dept_sorted(&self, dept: &str) -> Option<Vec<&String>> {
        self.get_employees_for_dept(dept).and_then(|mut employees| {
            employees.sort();
            Some(employees)
        })
    }

    pub fn modify_database(&mut self, cmd: &DbCommand) {
        match cmd {
            DbCommand::AddEmployee(employee) => {
                self.db
                    .entry(employee.dept.clone())
                    .or_insert(Vec::new())
                    .push(employee.name.clone());

                println!("{} added to {}.", employee.name, employee.dept);
            }
        }
    }

    pub fn parse_db_command(command_str: &str) -> Result<DbCommand> {
        let trimmed = command_str.trim_end_matches(|c| ",.!?\n".contains(c));

        if trimmed.is_empty() {
            return Err(Box::new(app_error::ApplicationError::new(
                "Command Required".to_string(),
                app_error::Kind::EmployeeDatabase,
            )));
        }

        let words: Vec<&str> = trimmed.split_ascii_whitespace().collect();

        match words[0].to_lowercase().as_str() {
            "add" => match words.iter().position(|&x| x.eq_ignore_ascii_case("to")) {
                Some(idx) => {
                    let name = words[1..idx].join(" ");
                    let dept = words[idx + 1..].join(" ");
                    Ok(DbCommand::AddEmployee(Database::make_employee(dept, name)))
                }
                None => Err(Box::new(app_error::ApplicationError::new(
                    format!("Invalid Add Syntax: [{}]", trimmed),
                    app_error::Kind::EmployeeDatabase,
                ))),
            },
            _ => Err(Box::new(app_error::ApplicationError::new(
                format!("Invalid modify command: [{}]", trimmed),
                app_error::Kind::EmployeeDatabase,
            ))),
        }
    }
}
