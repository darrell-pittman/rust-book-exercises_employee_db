use crate::employee_database::{DbCommand, EmployeeDatabase};
use std::io;

#[derive(Debug)]
enum Command {
    ShowAllEmployees,
    ShowEmployeesForDept(String),
    Modify(DbCommand),
    Quit,
    Invalid,
}

pub fn command_loop(database: &mut EmployeeDatabase) {
    loop {
        let command = get_command();
        match command {
            Command::ShowAllEmployees => print_all_employees(database),
            Command::ShowEmployeesForDept(dept) => print_employees_for_dept(database, &dept),
            Command::Modify(command) => database.modify_database(&command),
            Command::Quit => {
                println!("Good bye!");
                break;
            }
            _ => {
                println!("Invalid choice, try again!");
            }
        }
    }
}

fn get_command() -> Command {
    let mut choice = String::new();
    println!("Please enter a command:");
    println!("\t1 - Show All Employees");
    println!("\t2 - Show Employees for Dept");
    println!("\t3 - Add Employee");
    println!("\t4 - Quit");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().parse::<u32>() {
        Ok(num) => match num {
            1 => Command::ShowAllEmployees,
            2 => {
                println!("Please enter dept:");
                let mut dept = String::new();
                io::stdin()
                    .read_line(&mut dept)
                    .expect("Failed to readline");
                Command::ShowEmployeesForDept(dept.trim().to_string())
            }
            3 => {
                println!(r#"Please enter "Add Employee" command: (Add {{name}} to {{dept}})"#);
                let mut command = String::new();
                io::stdin()
                    .read_line(&mut command)
                    .expect("Failed to read line");
                let db_command = EmployeeDatabase::parse_db_command(&command[..]);
                Command::Modify(db_command)
            }
            4 => Command::Quit,
            _ => Command::Invalid,
        },
        Err(_) => Command::Invalid,
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
