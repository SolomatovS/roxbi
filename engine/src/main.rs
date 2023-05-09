use std::error::Error;
use dymod::dymod;
use models::{DataFrame, Transform, Config};


dymod! {
  pub mod transform {
    pub struct Extractor {
      fn init(name: &str) -> Box<dyn Transform>;
    }
  }
}


fn load_transform(path: &str) -> Result<Box<dyn Transform>, Box<dyn Error>> {
  let sub = transform::Extractor::load_library(&path)?;
  println!("loaded {:?}", &path);

  let res = sub.init("csv loader #1")?;
  Ok(res)
}

fn try_main() -> Result<(), Box<dyn Error>> {
  let csv_extractor = load_transform("/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libcsv_extractor.dylib")?;
  let sum_transform = load_transform("/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libsum_transform.dylib")?;

  let mut df = DataFrame::default();

  loop {
    df = csv_extractor.transform(df, None)?;
    df = sum_transform.transform(df, None)?;
    
    std::thread::sleep(std::time::Duration::from_millis(500));
  }
}


fn main() {
  try_main().unwrap();
}