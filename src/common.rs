use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub mod app_error {
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub enum Kind {
        EmployeeDatabase,
        Command,
    }

    impl fmt::Display for Kind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let kind_str = match *self {
                Kind::EmployeeDatabase => "Employee Database",
                Kind::Command => "Command",
            };
            write!(f, "{}", kind_str)
        }
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
            write!(f, "Kind: [{}], Msg: {}", self.kind, self.msg)
        }
    }

    impl Error for ApplicationError {}
}
