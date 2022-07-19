#[no_mangle]
pub extern fn count_sheep() -> u8 {
  println!("hello plugin");
  4
}