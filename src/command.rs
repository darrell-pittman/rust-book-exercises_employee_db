use crate::{
    common::{app_error, Result},
    employee_database::{Database, DbCommand},
};
use std::io;

#[derive(Debug)]
enum Command {
    ShowAllEmployees,
    ShowEmployeesForDept(String),
    Modify(DbCommand),
    Quit,
}

pub struct Application {
    database: Database,
}

impl Application {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub fn command_loop(&mut self) {
        loop {
            match Self::get_command() {
                Ok(command) => {
                    println!();
                    match command {
                        Command::ShowAllEmployees => self.print_all_employees(),
                        Command::ShowEmployeesForDept(dept) => self.print_employees_for_dept(&dept),
                        Command::Modify(command) => self.database.modify_database(command),
                        Command::Quit => {
                            println!("Good bye!");
                            break;
                        }
                    }
                }
                Err(e) => println!("\n{}", e.to_string()),
            }
            println!();
        }
    }

    fn get_command() -> Result<Command> {
        println!("Please enter a command:");
        println!("\t1 - Show All Employees");
        println!("\t2 - Show Employees for Dept");
        println!("\t3 - Add Employee");
        println!("\t4 - Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        if "q".eq_ignore_ascii_case(choice.trim()) {
            return Ok(Command::Quit);
        }

        let choice = choice.trim().parse::<u32>()?;

        match choice {
            1 => Ok(Command::ShowAllEmployees),
            2 => {
                println!("\nPlease enter dept:");
                let mut dept = String::new();
                io::stdin().read_line(&mut dept)?;
                Ok(Command::ShowEmployeesForDept(dept.trim().to_string()))
            }
            3 => {
                println!("\nPlease enter \"Add Employee\" command: (Add {{name}} to {{dept}})");
                let mut command = String::new();
                io::stdin().read_line(&mut command)?;
                let db_command = Database::parse_db_command(&command[..])?;
                Ok(Command::Modify(db_command))
            }
            4 => Ok(Command::Quit),
            _ => Err(Box::new(app_error::ApplicationError::new(
                format!("Unknown command choice: {}", choice),
                app_error::Kind::Command,
            ))),
        }
    }

    fn print_all_employees(&self) {
        println!();
        println!("----------All employees----------");
        println!();
        match self.database.get_departments_sorted() {
            Some(departments) => {
                for dept in departments {
                    self.print_employees_for_dept(&dept);
                }
            }
            None => println!("No data available!"),
        }
        println!();
    }

    fn print_employees_for_dept(&self, dept: &str) {
        println!("---------------------------------");
        println!();
        match self.database.get_employees_for_dept_sorted(dept) {
            Some(employees) => {
                println!("Department: {}", dept);
                println!("Employees: {:#?}", employees);
            }
            None => println!("No employees found for department {}", dept),
        }
        println!();
        println!("---------------------------------");
    }
}
