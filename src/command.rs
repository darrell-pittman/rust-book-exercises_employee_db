use crate::employee_database::{DbCommand, EmployeeDatabase, EmployeeDatabaseError};
use std::{fmt, io, num::ParseIntError};

#[derive(Debug)]
enum Command {
    ShowAllEmployees,
    ShowEmployeesForDept(String),
    Modify(DbCommand),
    Quit,
}

#[derive(Debug)]
struct CommandError {
    msg: String,
}

impl CommandError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        Self::new(err.to_string().as_str())
    }
}

impl From<ParseIntError> for CommandError {
    fn from(err: ParseIntError) -> Self {
        Self::new(err.to_string().as_str())
    }
}

impl From<EmployeeDatabaseError> for CommandError {
    fn from(err: EmployeeDatabaseError) -> Self {
        Self::new(err.to_string().as_str())
    }
}

type Result<T> = std::result::Result<T, CommandError>;

pub fn command_loop(database: &mut EmployeeDatabase) {
    loop {
        match get_command() {
            Ok(command) => {
                println!();
                match command {
                    Command::ShowAllEmployees => print_all_employees(database),
                    Command::ShowEmployeesForDept(dept) => {
                        print_employees_for_dept(database, &dept)
                    }
                    Command::Modify(command) => database.modify_database(command),
                    Command::Quit => {
                        println!("Good bye!");
                        break;
                    }
                }
            }
            Err(e) => println!("\n{}", e),
        }
        println!();
    }
}

fn get_command() -> self::Result<Command> {
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
            let db_command = EmployeeDatabase::parse_db_command(&command[..])?;
            Ok(Command::Modify(db_command))
        }
        4 => Ok(Command::Quit),
        _ => Err(CommandError::new(
            format!("Unknown choice: {}", choice).as_str(),
        )),
    }
}

pub fn print_all_employees(database: &EmployeeDatabase) {
    println!();
    println!("----------All employees----------");
    println!();
    if let Some(departments) = database.get_departments_sorted() {
        for dept in departments {
            print_employees_for_dept(database, &dept);
        }
    } else {
        println!("No data available!");
    }
    println!();
}

pub fn print_employees_for_dept(database: &EmployeeDatabase, dept: &String) {
    println!("---------------------------------");
    println!();
    if let Some(employees) = database.get_employees_for_dept_sorted(dept) {
        println!("Department: {}", dept);
        println!("Employees: {:#?}", employees);
    } else {
        println!("Deptartment {} not found!", &dept);
    }
    println!();
    println!("---------------------------------");
}
