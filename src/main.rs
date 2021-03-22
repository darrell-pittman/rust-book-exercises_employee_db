use command::Application;
use employee_database::Database;

mod command;
mod common;
mod employee_database;

fn main() {
    Application::new(Database::new()).command_loop();
}
