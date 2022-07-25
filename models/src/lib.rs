use std::fmt;

#[derive(Debug)]
pub struct DataFrame {
  
}

impl fmt::Display for DataFrame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.to_string())
  }
}