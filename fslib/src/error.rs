use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum SymbolError {
    SymbolNotFound,
}

impl SymbolError {
    fn message(&self) -> &str {
        match self {
            SymbolError::SymbolNotFound => {
                "symbol not found"
            }
            _ => "ERR",
        }
    }
}

impl fmt::Display for SymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message().fmt(f)
    }
}

impl Error for SymbolError {
    fn description(&self) -> &str {
        self.message()
    }
}
