use std::process::exit;
use dymod::dymod;
use models::DataFrame;

dymod! {
  pub mod transform {
    pub struct Extractor {
      fn to() -> DataFrame;
    }
  }
}

fn main() {
  pub use transform::Extractor;

  let file_path = "/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libextractors.dylib";

  let sub = match Extractor::load_lbrary(&file_path) {
    Ok(sub) => sub,
    Err(e) => {
      println!("{:?}", e);
      exit(1);
    },
  };

  println!("loaded dymod {:?}", &file_path);

  loop {
    let me = match sub.to() {
      Ok(me)=> me,
      Err(e) => {
        println!("{:?}", e);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        continue;
      },
    };

    println!("me: '{:?}'", me);
    std::thread::sleep(std::time::Duration::from_millis(500));
  }
}
