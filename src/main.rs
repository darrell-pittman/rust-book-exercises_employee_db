use employee_database::EmployeeDatabase;
mod command;
mod employee_database;

fn main() {
    let mut database = EmployeeDatabase::new();
    command::command_loop(&mut database);
}
