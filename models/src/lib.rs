// use std::fmt;
// use polars::prelude::*;
use std::error::Error;
pub use polars::prelude::DataFrame;
// #[derive(Debug)]
// pub struct DataFrame {
  
// }

// impl fmt::Display for DataFrame {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{:?}", self.to_string())
//   }
// }

pub trait Transform {
  fn transform(&self, from: DataFrame) -> Result<DataFrame, Box<dyn Error>>;
}