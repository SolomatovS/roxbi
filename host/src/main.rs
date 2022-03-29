#![allow(dead_code)]

mod helpers;

use std::{ffi::{OsString, OsStr}};

use file_system_library::file_system_repository_source::{FileSystemRepositorySource};
use ilibrary::{RepositoryLibrary};

fn main() {
    let mut source = FileSystemRepositorySource::new();

    let mut source = source
        //.add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        .read_dir(OsStr::new("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/"))
    ;
    /*
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        //.add_files_from_dir(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/"))
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
    ;
    */

    //let repo = RepositoryLibrary::new()
    //    .add_source(Box::new(source));

    //repo.build_missing_libs();
}