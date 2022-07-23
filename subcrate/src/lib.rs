#[no_mangle]
pub extern fn count_sheep() -> u8 {
  println!("hello plugin");
  4
}

pub struct TestTestovich {
  pub ert: u32,
}


#[no_mangle]
pub extern "Rust" fn new() -> TestTestovich {
  TestTestovich {
    ert: 222222,
  }
}
