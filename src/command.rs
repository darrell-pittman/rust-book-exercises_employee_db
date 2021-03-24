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

#[derive(Debug)]
enum CommandData {
    String(String),
    DbCommand(DbCommand),
    None,
}

impl Command {
    fn next(&self, data: CommandData) -> Result<Self> {
        match self {
            Command::Begin => Application::get_command_from_user(),
            Command::ShowAllEmployees => Ok(Command::Begin),
            Command::GetDepartment => Ok(Command::ShowEmployeesForDept(self.expect_string(data)?)),
            Command::ShowEmployeesForDept(_) => Ok(Command::Begin),
            Command::GetAddCommand => Ok(Command::Modify(self.expect_db_command(data)?)),
            Command::Modify(_) => Ok(Command::Begin),
            Command::Quit => Ok(Command::Quit),
        }
    }

    fn expect_string(&self, data: CommandData) -> Result<String> {
        match data {
            CommandData::String(string_data) => Ok(string_data),
            _ => self.data_error(),
        }
    }

    fn expect_db_command(&self, data: CommandData) -> Result<DbCommand> {
        match data {
            CommandData::DbCommand(command) => Ok(command),
            _ => self.data_error(),
        }
    }

    fn data_error<T>(&self) -> Result<T> {
        Err(Box::new(app_error::ApplicationError::new(
            format!("Wrong data for command: {:?}", self),
            app_error::Kind::System,
        )))
    }
}

pub struct Application {
    database: Database,
}

impl Application {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub fn command_loop(&mut self) {
        let mut next_command = Command::Begin;
        loop {
            next_command = match self.run_command(&next_command) {
                Ok(command) => {
                    if let Command::Quit = command {
                        break;
                    }
                    command
                }
                Err(e) => {
                    println!("\nError - {}", e);
                    Command::Begin
                }
            };
        }
    }

    fn run_command(&mut self, command: &Command) -> Result<Command> {
        let mut command_data = CommandData::None;
        match command {
            Command::Begin => {
                println!();
            }
            Command::ShowAllEmployees => {
                self.print_all_employees();
            }
            Command::GetDepartment => {
                println!("\nPlease enter dept:");
                let dept = Self::get_string_from_user()?;
                command_data = CommandData::String(dept);
                println!();
            }
            Command::ShowEmployeesForDept(dept) => {
                self.print_employees_for_dept(&dept);
            }
            Command::GetAddCommand => {
                print!("\nPlease enter \"Add Employee\" command: ");
                println!("(Add {{name}} to {{dept}})");
                let db_command = Self::get_string_from_user()?;
                let db_command = Database::parse_db_command(&db_command[..])?;
                command_data = CommandData::DbCommand(db_command);
                println!();
            }
            Command::Modify(command) => {
                self.database.modify_database(command);
            }
            Command::Quit => {
                println!("Good bye!");
            }
        }
        command.next(command_data)
    }

    fn get_string_from_user() -> Result<String> {
        let mut string_data = String::new();
        io::stdin().read_line(&mut string_data)?;
        let string_data = string_data.trim();
        if string_data.is_empty() {
            Err(Box::new(app_error::ApplicationError::new(
                "User input required".to_string(),
                app_error::Kind::Command,
            )))
        } else {
            Ok(string_data.trim().to_string())
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
