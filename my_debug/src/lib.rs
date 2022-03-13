#![allow(dead_code)]
#![allow(unused_macros)]

// The debug version
#[cfg(feature = "my_debug")]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(feature = "my_debug"))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {}
}
