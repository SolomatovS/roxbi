use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MyError {
    details: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
