use std::collections::HashMap;
use std::fmt;

pub struct EmployeeDatabase {
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

#[derive(Debug)]
pub struct EmployeeDatabaseError {
    msg: String,
}

impl EmployeeDatabaseError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for EmployeeDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

type Result<T> = std::result::Result<T, EmployeeDatabaseError>;

impl EmployeeDatabase {
    pub fn new() -> EmployeeDatabase {
        EmployeeDatabase { db: HashMap::new() }
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

    pub fn modify_database(&mut self, cmd: DbCommand) {
        match cmd {
            DbCommand::AddEmployee(employee) => {
                let msg = format!("{} added to {}.", employee.name, employee.dept);

                self.db
                    .entry(employee.dept)
                    .or_insert(Vec::new())
                    .push(employee.name);

                println!("{}", msg);
            }
        }
    }

    pub fn parse_db_command(command_str: &str) -> self::Result<DbCommand> {
        let words: Vec<&str> = command_str
            .trim_end_matches(|c| ",.!?\n".contains(c))
            .split_ascii_whitespace()
            .collect();

        match words[0].to_lowercase().as_str() {
            "add" => {
                if let Some(idx) = words.iter().position(|&x| x.eq_ignore_ascii_case("to")) {
                    let name = words[1..idx].join(" ");
                    let dept = words[idx + 1..].join(" ");
                    Ok(DbCommand::AddEmployee(EmployeeDatabase::make_employee(
                        dept.to_string(),
                        name.to_string(),
                    )))
                } else {
                    Err(EmployeeDatabaseError::new(
                        format!("Invalid Add Syntax: [{}]", command_str.trim()).as_str(),
                    ))
                }
            }
            _ => Err(EmployeeDatabaseError::new(
                format!("Invalid modify command: [{}]", command_str.trim()).as_str(),
            )),
        }
    }
}
