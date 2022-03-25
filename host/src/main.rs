#![allow(dead_code)]

mod helpers;

use std::{ffi::OsString};

use file_system_library::file_system_repository_source::{FileSystemRepositorySource};
use ilibrary::{RepositoryLibrary};

fn main() {
    let source = FileSystemRepositorySource::new()
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        .add_file_path_and_action_if_build_error(
            OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1"),
            Box::new(|e| {
                println!("{}", &e);
            })
        )
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
    ;

    let repo = RepositoryLibrary::new()
        .add_source(Box::new(source));

    repo.generate_and_update_libs();
}