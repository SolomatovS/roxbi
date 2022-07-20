use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum DymodError {
    SymbolNotFound,
}

impl DymodError {
    fn message(&self) -> &str {
        match self {
          DymodError::SymbolNotFound => {
                "symbol not found"
            }
            _ => "ERR",
        }
    }
}

impl fmt::Display for DymodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message().fmt(f)
    }
}

impl Error for DymodError {
    fn description(&self) -> &str {
        self.message()
    }
}
