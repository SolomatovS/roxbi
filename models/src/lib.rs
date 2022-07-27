use std::error::Error;
pub use polars::prelude::DataFrame;
pub use config::Config;


pub trait Transform {
  fn transform(&self, from: DataFrame, conf: Option<&Config>) -> Result<DataFrame, Box<dyn Error>>;
}