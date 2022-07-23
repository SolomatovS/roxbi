use std::process::exit;

use dymod::dymod;

dymod! {
  #[struct = Subcrate]
  pub mod subcrate {
    fn count_sheep() -> u8;
  }
}

fn main() {
  let sub = match subcrate::load("/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libsubcrate.dylib") {
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
