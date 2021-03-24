use crate::{
    common::{app_error, Result},
    employee_database::{Database, DbCommand},
};
use std::io;

#[derive(Debug)]
enum Command {
    Begin,
    ShowAllEmployees,
    GetDepartment,
    ShowEmployeesForDept(String),
    GetAddCommand,
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
        let mut next_command: Result<Command> = Self::get_command_from_user();
        loop {
            next_command = match next_command {
                Ok(command) => self.run_command(&command),
                Err(e) => {
                    println!("\nError - {}", e);
                    Ok(Command::Begin)
                }
            };
            if let Ok(Command::Quit) = next_command {
                break;
            }
        }
    }

    fn run_command(&mut self, command: &Command) -> Result<Command> {
        match command {
            Command::Begin => {
                println!();
                Self::get_command_from_user()
            }
            Command::ShowAllEmployees => {
                self.print_all_employees();
                Ok(Command::Begin)
            }
            Command::GetDepartment => {
                println!("\nPlease enter dept:");
                let mut dept = String::new();
                io::stdin().read_line(&mut dept)?;
                println!();
                Ok(Command::ShowEmployeesForDept(dept.trim().to_string()))
            }
            Command::ShowEmployeesForDept(dept) => {
                self.print_employees_for_dept(&dept);
                Ok(Command::Begin)
            }
            Command::GetAddCommand => {
                print!("\nPlease enter \"Add Employee\" command: ");
                println!("(Add {{name}} to {{dept}})");
                let mut command = String::new();
                io::stdin().read_line(&mut command)?;
                let db_command = Database::parse_db_command(&command[..])?;
                println!();
                Ok(Command::Modify(db_command))
            }
            Command::Modify(command) => {
                self.database.modify_database(command);
                Ok(Command::Begin)
            }
            Command::Quit => {
                println!("Good bye!");
                Ok(Command::Quit)
            }
        }
    }

    fn get_command_from_user() -> Result<Command> {
        println!("Please enter a command:");
        println!("\t1 - Show All Employees");
        println!("\t2 - Show Employees for Dept");
        println!("\t3 - Add Employee");
        println!("\t4 - Quit (q)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        if "q".eq_ignore_ascii_case(choice.trim()) {
            return Ok(Command::Quit);
        }

        match choice.trim().parse::<u32>()? {
            1 => Ok(Command::ShowAllEmployees),
            2 => Ok(Command::GetDepartment),
            3 => Ok(Command::GetAddCommand),
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
