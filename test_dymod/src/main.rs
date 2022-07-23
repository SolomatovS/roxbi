use std::process::exit;

use dymod::dymod;

dymod! {
  pub mod subcrate {
    pub struct Subcrate {
      fn count_sheep() -> u8;
    }
  }
}

fn main() {
  pub use subcrate::Subcrate;

  let sub = match Subcrate::load("/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libsubcrate.dylib") {
    Ok(sub) => sub,
    Err(e) => {
      println!("error load library: {}", &e);
      exit(1);
    },
  };

  loop {
    let message = sub.count_sheep().unwrap();
    println!("There are '{}' sheep.", message);
    std::thread::sleep(std::time::Duration::from_millis(2000));
  }
}
