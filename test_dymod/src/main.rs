use dymod::dymod;

dymod! {
  #[path = "/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libsubcrate.dylib"]
  pub mod subcrate {
    fn count_sheep() -> u8;
  }
}

fn main() {
    loop {
        let message = subcrate::count_sheep();
        println!("There are '{}' sheep.", message);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
