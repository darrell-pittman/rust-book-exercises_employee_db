use std::collections::HashMap;

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
    Invalid(String),
}

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
        if let Some(mut depts) = self.get_departments() {
            depts.sort();
            Some(depts)
        } else {
            None
        }
    }

    pub fn get_employees_for_dept(&self, dept: &str) -> Option<Vec<&String>> {
        if let Some(employees) = self.db.get(dept) {
            Some(employees.iter().collect())
        } else {
            None
        }
    }

    pub fn get_employees_for_dept_sorted(&self, dept: &str) -> Option<Vec<&String>> {
        if let Some(mut employees) = self.get_employees_for_dept(dept) {
            employees.sort();
            Some(employees)
        } else {
            None
        }
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
            _ => println!(
                "Invalid, unknown or unsupported database command: {:?}",
                cmd
            ),
        }
    }

    pub fn parse_db_command(command_str: &str) -> DbCommand {
        let words: Vec<&str> = command_str
            .trim_end_matches(|c| ",.!?\n".contains(c))
            .split_ascii_whitespace()
            .collect();

        match words[0].to_lowercase().as_str() {
            "add" => {
                if let Some(idx) = words.iter().position(|&x| x.eq_ignore_ascii_case("to")) {
                    let name = words[1..idx].join(" ");
                    let dept = words[idx + 1..].join(" ");
                    DbCommand::AddEmployee(EmployeeDatabase::make_employee(
                        dept.to_string(),
                        name.to_string(),
                    ))
                } else {
                    DbCommand::Invalid(command_str.trim().to_string())
                }
            }
            _ => DbCommand::Invalid(command_str.trim().to_string()),
        }
    }
}
