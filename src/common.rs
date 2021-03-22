use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub mod app_error {
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub enum Kind {
        EmployeeDatabase,
        Command,
    }

    #[derive(Debug)]
    pub struct ApplicationError {
        msg: String,
        kind: Kind,
    }

    impl ApplicationError {
        pub fn new(msg: String, kind: Kind) -> Self {
            Self { msg, kind }
        }
    }

    impl fmt::Display for ApplicationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    impl Error for ApplicationError {}
}
