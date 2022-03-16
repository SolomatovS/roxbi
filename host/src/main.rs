#![allow(dead_code)]

mod helpers;

use std::ffi::OsString;

use file_system_library::file_system_repository_source::FileSystemRepositorySource;

fn main() {
    let mut file_source = FileSystemRepositorySource::new();

    let file_source = file_source
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1"))
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
    ;
    
    let libs = file_source.build(|e| {
        println!("{}", &e);
    });

    libs.iter().for_each(|x| println!("{}", x));
}