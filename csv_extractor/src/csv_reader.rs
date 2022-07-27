use std::result::Result;
use std::error::Error;
use polars::prelude::{DataFrame, CsvReader, SerReader};
use models::{Transform};


pub struct CsvReaderTransform {

}

impl Transform for CsvReaderTransform {
  fn transform(&self, #[allow(unused_variables)] data: DataFrame) -> Result<DataFrame, Box<dyn Error>> {

    let path = String::from("/Users/solomatovs/Documents/GitHub/roxbi/csv_extractor/example/sdfsdf.csv");

    let csv_reader = CsvReader::from_path(&path)?
      .has_header(true)
      .with_delimiter(b',')
      .finish()?
    ;

    println!(r#"{:?} -> {:?}"#, path, &csv_reader);

    Ok(csv_reader)
  }
}

#[no_mangle]
pub extern "Rust" fn init() -> Box<dyn Transform> {
  Box::new(CsvReaderTransform {})
}