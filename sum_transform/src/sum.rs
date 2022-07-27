use polars::prelude::*;
use models::{Transform};

pub struct SumTransform {

}

impl Transform for SumTransform {
  fn transform(&self,
    #[allow(unused_variables)] data: DataFrame
  ) -> std::result::Result<DataFrame, Box<dyn std::error::Error>> {

    println!("{:?}", &data);

    Ok(data)
  }
}

#[no_mangle]
pub extern "Rust" fn init() -> Box<dyn Transform> {
  Box::new(SumTransform {})
}