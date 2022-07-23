use std::process::exit;
use std::fmt;
use dymod::dymod;

#[derive(Debug)]
pub struct TestTestovich {
  pub ert: u32,
}

impl fmt::Display for TestTestovich {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

dymod! {
  pub mod subcrate {
    pub struct Subcrate {
      fn new() -> u8;
      fn count_sheep() -> u8;
    }
  }
}

fn main() {
  pub use subcrate::Subcrate;

  let sub = match Subcrate::load_lbrary("/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libsubcrate.dylib") {
    Ok(sub) => sub,
    Err(e) => {
      println!("error load library: {}", &e);
      exit(1);
    },
  };

  loop {
    let me = match sub.new() {
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
