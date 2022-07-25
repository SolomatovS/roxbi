// use rand::{thread_rng, Rng};
use models::DataFrame;

#[no_mangle]
pub extern "Rust" fn from(data: &DataFrame) -> DataFrame {
  println!("from");
  println!("{:?}", &data);
  DataFrame {}
}

#[no_mangle]
pub extern "Rust" fn to() -> DataFrame {
  println!("to");

  DataFrame {}
}

#[no_mangle]
pub extern "Rust" fn load() {

}