use std::result::Result;
use std::error::Error;
use models::{DataFrame, Transform, Config};

pub struct SumTransform {

}

impl Transform for SumTransform {
  fn transform(&self, data: DataFrame, conf: Option<&Config>) -> Result<DataFrame, Box<dyn Error>> {
    println!("conf: {:?}", &conf);
    println!("data: {:?}", &data);
    Ok(data)
  }
}

#[no_mangle]
pub extern "Rust" fn init() -> Box<dyn Transform> {
  Box::new(SumTransform {})
}