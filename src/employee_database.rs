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

    pub fn print_all_employees(&self) {
        println!();
        println!("----------All employees----------");
        println!();
        if self.db.is_empty() {
            println!("No data available!");
        } else {
            for dept in self.db.keys() {
                self.print_employees_for_dept(&dept);
            }
        }
        println!();
    }

    pub fn print_employees_for_dept(&self, dept: &String) {
        println!("---------------------------------");
        println!();
        if let Some(employees) = self.db.get(dept) {
            println!("Department: {}", dept);
            println!("Employees: {:#?}", employees);
        } else {
            println!("Deptartment {} not found!", &dept);
        }
        println!();
        println!("---------------------------------");
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
            _ => println!(
                "Invalid, unknown or unsupported database command: {:?}",
                cmd
            ),
        }
    }

    pub fn parse_db_command(command_str: &str) -> DbCommand {
        let words: Vec<&str> = command_str.split_ascii_whitespace().collect();
        match words[0].to_lowercase().as_str() {
            "add" => {
                if let Some(idx) = words.iter().position(|&x| x.eq_ignore_ascii_case("to")) {
                    let name = &words[1..idx].join(" ");
                    let dept = &words[idx + 1..].join(" ");
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
